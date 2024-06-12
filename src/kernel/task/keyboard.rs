use crate::{print, println};
use crate::menu::{calculator_over,findaddress_over,menu_begin, menu_welcome,txt_test_over};

use alloc::string::ToString;
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

pub fn check_keypresses(x: char) -> i32 {
    let mut y=-1;

    if x >= '0' && x <= '9' {
        y=0;
    } 
    else if x == ',' {
        y=1;
    }
    else if x == '.' {
        y=2;
    }
    else if x == ';' {
        y=3;
    }
    else if x == '/' {
        y=4;
    }
    else if x>='a'&&x<='z' {
        y=64;
    }
    return y;
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    menu_begin();

    let mut cnt=0;
    let mut op1=0;

    let mut sum=0;
    let mut op2=1;

    let mut adr=[0;10];


    let mut ck1=0;
    let mut ck2=0;
    let mut ck3=0;
    let mut ck4=0;
    let mut ck5=0;

    let mut s:u64=0;

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        print!("{}", character);

                        if cnt==0 && (character=='1'||character=='2')   {
                            op1=(character as i32)-48;
                        }

                        if cnt==0{
                            menu_welcome(op1);
                        }

                        cnt+=1;

                        if cnt!=1 && op1==1 && check_keypresses(character)==0 {
                            if op2==1  {
                                sum=sum+((character as i32)-48);
                            }
                            else if op2==2 {
                                sum=sum-((character as i32)-48);
                            }
                            else if op2==3 {
                                sum=sum*((character as i32)-48);
                            }
                            else if op2==4 {
                                sum=sum/((character as i32)-48);
                            }
                        }
                        else if cnt!=1 && op1==1 && character=='=' {
                            calculator_over(sum);
                            ck1=0;ck2=0;ck3=0;ck4=0;ck5=0;
                            cnt=0;op1=0;sum=0;op2=1;s=0;
                        }
                        else if cnt!=1 && op1==1 {
                            op2=check_keypresses(character);
                        }
                        
                        else if cnt !=1 && op1==2 {
                            if check_keypresses(character)==0 {
                                s=s*16+(character as u64)-48;
                            }
                            else if check_keypresses(character)==64 {
                                s=s*16+(character as u64)-('a' as u64)+10;
                            }

                            if cnt==9 {
                                unsafe {
                                    *(s as *mut u64) = 42;
                                }
                                findaddress_over();
                                ck1=0;ck2=0;ck3=0;ck4=0;ck5=0;
                                cnt=0;op1=0;sum=0;op2=1;s=0;
                            }
                        }
                        
                        
                        
                        
                        else if cnt!=0 && character == 'e'{
                            ck1=1;
                            ck2=0;ck3=0;ck4=0;ck5=0;
                        }
                        else if cnt!=0 && character == 'x' && ck1==1{
                            ck2=1;
                            ck3=0;ck4=0;ck5=0;
                        }
                        else if cnt!=0 && character == 'i' && ck2==1{
                            ck3=1;
                            ck4=0;ck5=0;
                        }
                        else if cnt!=0 && character == 't' && ck3==1{
                            ck4=1;
                            ck5=0;
                        }
                        else if cnt!=0 && character == '\n' && ck4==1{
                            ck5=1;
                        }
                        else if cnt!=0
                        {
                            ck1=0;ck2=0;ck3=0;ck4=0;ck5=0;
                        }

                        if ck5==1{
                            txt_test_over();
                            ck1=0;ck2=0;ck3=0;ck4=0;ck5=0;
                            cnt=0;op1=0;sum=0;op2=1;s=0;
                        }

                    }
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
}