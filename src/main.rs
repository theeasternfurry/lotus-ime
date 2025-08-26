use std::fs::File;
use std::ffi::OsString;
use std::path::Path;

use daemonize_me::{Daemon, Group, User};
use users::get_current_username;
use wayland_client::{protocol::wl_registry, Connection, Dispatch, QueueHandle};

const APP_NAME: &str = "lotus-ime";

struct AppData;

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        _state: &mut Self,
        _: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppData>,
    ) {
        // When receiving events from the wl_registry, we are only interested in the
        // `global` event, which signals a new available global.
        // When receiving this event, we just print its characteristics in this example.
        if let wl_registry::Event::Global { name, interface, version } = event {
            println!("[{}] {} (v{})", name, interface, version);
        }
    }
}

fn init_dir(local_share_dir: &str) {
    if !Path::new(&local_share_dir).exists() {
        std::fs::create_dir_all(&local_share_dir).expect("Failed to create local share directory");
    }

    if !Path::new(&format!("{}/{}", local_share_dir, APP_NAME)).exists() {
        std::fs::create_dir_all(&format!("{}/{}", local_share_dir, APP_NAME)).expect("Failed to create logs directory");
    }

    if !Path::new(&format!("{}/{}/logs", local_share_dir, APP_NAME)).exists() {
        std::fs::create_dir_all(&format!("{}/{}/logs", local_share_dir, APP_NAME)).expect("Failed to create logs directory");
    }
}

fn main() {
    // Create a Wayland connection by connecting to the server through the
    // environment-provided configuration.
    let conn = Connection::connect_to_env().unwrap();

    // Retrieve the WlDisplay Wayland object from the connection. This object is
    // the starting point of any Wayland program, from which all other objects will
    // be created.
    let display = conn.display();

    // Create an event queue for our event processing
    let mut event_queue = conn.new_event_queue();
    // And get its handle to associate new objects to it
    let qh = event_queue.handle();

    // Create a wl_registry object by sending the wl_display.get_registry request.
    // This method takes two arguments: a handle to the queue that the newly created
    // wl_registry will be assigned to, and the user-data that should be associated
    // with this registry (here it is () as we don't need user-data).
    let _registry = display.get_registry(&qh, ());

    // At this point everything is ready, and we just need to wait to receive the events
    // from the wl_registry. Our callback will print the advertised globals.
    println!("Advertised globals:");

    // To actually receive the events, we invoke the `roundtrip` method. This method
    // is special and you will generally only invoke it during the setup of your program:
    // it will block until the server has received and processed all the messages you've
    // sent up to now.
    //
    // In our case, that means it'll block until the server has received our
    // wl_display.get_registry request, and as a reaction has sent us a batch of
    // wl_registry.global events.
    //
    // `roundtrip` will then empty the internal buffer of the queue it has been invoked
    // on, and thus invoke our `Dispatch` implementation that prints the list of advertised
    // globals.
    event_queue.roundtrip(&mut AppData).unwrap();

    let username: OsString = get_current_username().expect("Failed to get current username");
    let home_dir = format!("/home/{}", username.to_string_lossy());
    let local_share_dir = format!("{}/.local/share", home_dir);

    init_dir(&local_share_dir);

    let stdout = File::create(format!("{}/{}/logs/{}-info.log", local_share_dir, APP_NAME, APP_NAME)).unwrap();
    let stderr = File::create(format!("{}/{}/logs/{}-error.log", local_share_dir, APP_NAME, APP_NAME)).unwrap();
    let daemon = Daemon::new()
        .pid_file("lotus-ime.pid", Some(false))
        .user(User::try_from(username.to_string_lossy().as_ref()).unwrap())
        .group(Group::try_from("users").unwrap())
        .umask(0o000)
        .work_dir(format!("/home/{}/.local/share/logs/lotus-ime", username.to_string_lossy()))
        .stdout(stdout)
        .stderr(stderr)
        .start();

    match daemon {
        Ok(_) => println!("Daemonized with success"),
        Err(e) => eprintln!("Error, {}", e),
    }

    loop {
        // You wil have to kill this process yourself
    }
}
