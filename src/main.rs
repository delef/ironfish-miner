mod rpc;
use rpc::Ipc;


#[tokio::main(flavor = "current_thread")]
async fn main() {
	let path = "/Users/delef/Projects/crypto/.ironfish/ironfish.ipc";
	let mut client = Ipc::connect(path).await;
	
	client.request("miner/newBlocksStream").await;
	client.stream(|stream| {
		println!("{:?}", stream);
	}).await;
}