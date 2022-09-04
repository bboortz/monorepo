#[macro_use]
extern crate log;

use std::thread;
use std::time::Duration;

use fltk::{app, enums::FrameType, prelude::*, *};

mod conntest;
mod error;
mod reqwest_client;
mod setup;
#[cfg(target_os = "windows")]
mod systray;
use async_std::{
    prelude::*, // 1
    task, // 2
};
use futures::channel::mpsc; // 1
use futures::sink::SinkExt;
use std::sync::Arc;

type Sender<T> = mpsc::UnboundedSender<T>; // 2
type Receiver<T> = mpsc::UnboundedReceiver<T>;

const TIMEOUT: std::time::Duration = Duration::from_millis(10000);

async fn str_len_async(s: &str) -> usize {
  // do something awaitable ideally... 
  s.len()
}

async fn app() {
    loop {
        match conntest::run() {
						Ok(val) => {
							info!("REPORT: {:?}", val);
						},
						Err(e) => {
								error!("{}", e);
						}
				}
        thread::sleep(TIMEOUT);
    }
}

async fn connection_writer_loop(mut messages: Receiver<String>, stream: Arc<TcpStream>) -> Result<()> {
    let mut stream = &*stream;
    while let Some(msg) = messages.next().await {
        stream.write_all(msg.as_bytes()).await?;
    }
    Ok(())
}

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<(), error::Error>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}


fn run() -> Result<(), error::Error> {
    setup::setup();

/*
		let rt = tokio::runtime::Runtime::new().unwrap();
    let future = app();
    rt.spawn(future);
*/
		// info!("{}", str_len_async("x5ff").await);
    let app = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let mut frame = frame::Frame::new(10, 10, 380, 200, "");
    frame.set_frame(FrameType::EngravedBox);
    let mut but = button::Button::new(160, 220, 80, 40, "Click me!");
    win.end();
    win.show();

    but.set_callback(move |_| {
//        testconn::test_conn();
        match conntest::run() {
						Ok(val) => {
							info!("REPORT: {:?}", val);
						},
						Err(e) => {
								error!("{}", e);
						}
				}
        frame.set_label("Hello world!")
    });
    //     but.set_callback(move |_| test_conn(&frame) );

    #[cfg(target_os = "windows")]
    {
        unsafe {
            WINDOW = win.raw_handle();
        }
        win.set_callback(|w| {
            // We intercept the closing of the window here
            unsafe {
                w.platform_hide();
            }
        });
        use crate::systray::NativeUi;
        systray::init().expect("Failed to init Native Windows GUI");
        let _ui = systray::SystemTray::build_ui(Default::default()).expect("Failed to build UI");
        systray::dispatch_thread_events_with_callback(move || {
            if win.shown() {
                app.run().unwrap();
            } else {
                app::sleep(0.030);
            }
        });
    }

    #[cfg(not(target_os = "windows"))]
    app.run().unwrap();

    loop {
        conntest::run()?;
        thread::sleep(TIMEOUT);
    }

    // Ok(())
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    }
}
