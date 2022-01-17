use std::{thread, time};

use psrt::client::{
    Client as PsrtClient,
    Config as PsrtConfig,
};
use tokio::time::sleep;

// временно тут
#[derive(Debug)]
pub struct Rpc {
    tcp: bool,
    host: String,
    port: u8,
    socket_path: String,
}

// временно тут
#[derive(Debug)]
pub struct Config {
    rpc: Rpc,
}

#[derive(Debug)]
pub struct Miner {
    threads: i8,
    config: Config,
}

impl Miner {
    pub fn new(threads: i8, socket_path: String) -> Self {
        Miner {
            threads: threads,
            config: Config {
                rpc: Rpc {
                    tcp: false,
                    host: String::new(),
                    port: 0,
                    socket_path: socket_path,
                }
            }
        }
    }

    pub async fn start(&self) {
        //      const client = this.sdk.client
        //      const miner = new IronfishMiner(flags.threads)

        if self.threads == 0 || self.threads < -1 {
            println!("--threads must be a positive integer or -1.");
            return
        }

        self.client().await;

        loop {
            let connected: bool = false;//client.tryConnect();

            if !connected {
                // log: Not connected to a node - waiting 5s before retrying
                println!("Not connected to a node - waiting 5s before retrying");
                thread::sleep(time::Duration::from_secs(5));
                continue;
            }

            println!("THE END!");
        }
    }

    async fn client(&self) {
        println!("{}", self.config.rpc.socket_path.as_str());

        let cfg = PsrtConfig::new(self.config.rpc.socket_path.as_str())
            .set_timeout(time::Duration::from_secs(5))
            .build();

        // connect PSRT client
        let mut client = PsrtClient::connect(&cfg).await.expect("Failed to connect");

        // subscriptions
        client.subscribe("miner/newBlocksStream".to_owned()).await.unwrap();
        client.subscribe("miner/successfullyMined".to_owned()).await.unwrap();

        // get data channel
        let data_channel = client.take_data_channel().unwrap();

        let receiver_fut = tokio::spawn(async move {
            // receive messages from the server
            while let Ok(message) = data_channel.recv().await {
                println!(
                    "topic: {}, data: {}",
                    message.topic(),
                    message.data_as_str().unwrap()
                );
            }
        });

        client.bye().await.unwrap();
        receiver_fut.await.unwrap();


        // let client: IpcClient;

        // const client = new IronfishIpcClient(
        //     config.get('enableRpcTcp')
        //       ? {
        //           mode: 'tcp',
        //           host: config.get('rpcTcpHost'),
        //           port: config.get('rpcTcpPort'),
        //         }
        //       : {
        //           mode: 'ipc',
        //           socketPath: config.get('ipcPath'),
        //         },
        //     logger,
        //     config.get('rpcRetryConnect'),
        //   )
    }
}