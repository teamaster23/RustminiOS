use crate::println;



pub fn menu_init()
{
    println!(".__           .__  .__                               .__       .___");
    println!("|  |__   ____ |  | |  |   ____   __  _  _____________|  |    __| _/");
    println!("|  |  |_/ __ ||  | |  |  /  _ |  | |/ |/ /  _ |_  __ |  |   / __ | ");
    println!("|   Y  |  ___/|  |_|  |_(  <_> )  |     (  <_> )  | |/  |__/ /_/ | ");
    println!("|___|  /|___  >____/____/|____/ /| |/|_/ |____/|__|  |____/|____ | ");
    println!("     |__|    |__|                |__|                          |__| ");
}

pub fn menu_begin()
{
    println!("Enter some characters,");
    println!("if your first character is 1,go to calculator");
    println!("if your first character is 2,go to findaddress");
    println!("if your first character is others,go to txt_test");
}

pub fn menu_welcome(x:i32)
{
    if x==1
    {
        println!("\n welcome to calculator");
    }
    else if x==2
    {
        println!("\n welcome to findaddress");
    }
    else
    {
        println!("\n welcome to txt_test");
    }
}

pub fn calculator_over(x:i32)
{
    println!("{}",x);
    println!("Now you have exitted the calculator.\nIf you want to use calculator,please press 1;if you want to find address,please press 2;else press any key to go back to txt_test.");
}

pub fn txt_test_over()
{
    println!("Now you have exitted the txt_test.\nIf you want to use calculator,please press 1;if you want to find address,please press 2;else press any key to go back to txt_test.");
}

pub fn findaddress_over()
{
    println!("\nThis address can be in.\nIf you want to use calculator,please press 1;if you want to find address,please press 2;else press any key to go back to txt_test.");
}

