
mod app;
mod config;

use tokio::net::TcpListener;
use config::AppConfig;

// Runtim entry
#[tokio::main]
async fn main()->anyhow::Result<()>{
// async => fu can PAUSE without blocking thread. 
// 1.the function will pause in bg 2.other function will run 3.once it get everyting it will run.
// use on -> Db, Network, Disk, Other services
//
// async fn -> when called => it does not run it creates a FUTURE.
// a future is => state of mashine (Not started, waiting for something, Ready) => noting run unless someting polls it.
// so who polls it? => here the line #[tokio::main] -> create EXICUTOR.
// Exicutor => repetadly polls future, park task when waiting, wakes when ready.
//
//
// Y: async


    let config=AppConfig::from_env()?;


    let app = app::build_app(); //file app -> function build_app
    let listener= TcpListener::bind(&config.server_addr).await?;
    // await can ONLY used on something that might take time. eg. Network, socket, timers, file I/O
    // listner say 'i need this port 👆', OS can say => {delay, fail, wait for other resources} ➡️ await.
    // await suspends the current async task until the awaited future makes progress
    // Y: await
    // G: So the function is paused till OS binds the port.

    println!("⚡ VAT on {}",&config.server_addr);
    axum::serve(listener,app).await?;
    // Serving requests is an async operation that never finishes (until shutdown) ➡  await
    // the .await? => shows errors which returns => so the funciton 
    // Y: await?
    //
    // will show error with help of main() -> anyhow::Resualt<()>
    // Y: anyhow

    Ok(()) // when noting usefull to return.
    
}

// ----------------------------------------------------------------------------|
// We call TcpListener::bind(...) await? → returns a Future
// We .await it → Tokio polls it
//
// OS says: “Port not ready yet” or “I’m working”
// Tokio: pauses this task | runs other tasks
// OS signals completion => Tokio wakes this future .await returns TcpListener
