use std::rc::Rc;
use std::thread;
use std::sync::{Arc, mpsc, Mutex, RwLock};
#[test]
fn main() {
    //线程之间消息通信,tx发送端，接收端
/*    let (tx, rc) = mpsc::channel();*/
    //异步消息，没有堵塞，里面的1代表缓存的消息数量，满了之后触发阻塞，后面消息无法传入
    //使用异步消息虽然能非常高效且不会造成发送线程的阻塞，
    // 但是存在消息未及时消费，最终内存过大的问题。在实际项目中，
    // 可以考虑使用一个带缓冲值的同步通道来避免这种风险
    //let (t, r) = mpsc::sync_channel(1);
    //克隆一个发送端
/*    let tx2 = tx.clone();
    thread::spawn(move|| {
        let v = vec![1,2,3,4,5,6];
        for num in v {
            tx.send(num).unwrap();
        }
    });
    thread::spawn(move|| {
        let v = vec![1,2,3,4,5,6];
        for num in v {
            tx2.send(num).unwrap();
        }
    });
    //连续接收
    for r in rc {
        println!("接收端{}", r)
    }*/


    //创建互斥锁
/*    let m = Mutex::new(5);
    {
        let mut mx = m.lock().unwrap();
        *mx = 6;
    }
    println!("{:?}", m);*/

    //多所有权
/*    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move|| {
            //获取锁
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        //将线程放入集合
        handles.push(handle);
    }
    //等待所有子线程执行完
    for handle in handles {
        handle.join().unwrap();
    }
    //获取锁但是得等上面的执行完
    println!("结果 {}", *counter.lock().unwrap())*/


    //大量数据读写锁
    let lock = RwLock::new(0);
    //同一时间多读
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("{} {}", r1, r2);
    }
    //唯一写
    {
        //获取带读写锁的数据
        //获取唯一写
        let mut w = lock.write().unwrap();
        *w += 1;
    }
}