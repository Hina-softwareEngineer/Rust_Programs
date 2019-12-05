use std::io;
fn main(){
    println!("Enter a Number: ");

    let mut no=String::new();
    io::stdin().read_line(&mut no).expect("Failed to read");
    let no1=no.trim().parse::<u64>().unwrap();
    let result=is_armstrong_no(no1);
    if (result==no1){
        println!("{} is an Armstrong Number.",no1);
    }
    else{
        println!("{} is not  an Armstrong Number.",no1);
    }

}
fn is_armstrong_no(mut num:u64)->u64{
    let mut temp=num;
    let mut sum:u64=0;
    let mut rem:u64;
    let mut count:u64=0;
    while(temp>0){
        temp=temp/10;
        count=count+1;
    }
    println!("coutn:{}",count);
    while(num>0){
        rem=num%10;
        sum=sum+power(rem,count);
        println!("{}",sum);
        num=num/10;
    }
    println!("{}",sum);
    sum
}
fn power(r:u64,mut c:u64)->u64{
    let mut pow:u64=1;
    while (c>0){
        pow=r*pow;
        println!("{}",pow);
        c=c-1;
    }
    println!("{}",pow);
    pow
}
