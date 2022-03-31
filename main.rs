use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

//number of threads
const THREAD_NUM: usize = 4;
const THRESHOLD: usize = 10;

//example given function 
fn f(t: i32) -> i32 {
	t * 10
}

fn main() {
	//example vector
	let v = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];
	let size = v.len();
	let mut result: Vec<i32>= vec![];
	
	if size >= THRESHOLD {
		let vector: Arc<Vec<i32>> = Arc::new(v);
		//channel
		let (tx, rx) = mpsc::channel();
		
		//size of block for each thread
		let block_size = vector.len() / THREAD_NUM;
		//final block size
		let last_block = vector.len() % THREAD_NUM;
		
		for id in 0..THREAD_NUM {
			let thread_vector = vector.clone();  
			let thread_tx = tx.clone();
			//creating threads; each thread process one disjoint block of vector
			thread::spawn(move || {
					let pos = id * block_size;
					for val in thread_vector[pos..pos + block_size].iter().cloned() {
						let x = f(val);
						//send message with result to main thread
						thread_tx.send(x).unwrap();
					}
				});
		}
		drop(tx);
		
		//last block is processed by main thread
		let pos = size - last_block;
		for val in &vector[pos..size]{
			result.push(f(*val));
		}	
		//recieve messages
		for received in rx {
			result.push(received);
		}
	}
	else {
		for val in v{
			result.push(f(val));
		}
	}
    for val in result{
		print!(" {\t}",val);
	}
	println!("");

}




