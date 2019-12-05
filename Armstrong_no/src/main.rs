use std::io;
fn main(){
    println!("Enter a Number: ");

    let mut no=String::new();
    io::stdin().read_line(&mut no).expect("Failed to read");
    let no1=no.trim().parse::<u32>().unwrap();

    let result=is_armstrong_no(no1);


    if (result==no1){
        println!("{} is an Armstrong Number.",no1);
    }
    else{
        println!("{} is not  an Armstrong Number.",no1);
    }

}
fn is_armstrong_no(mut num:u32)->u32{
    let mut temp=num;
    let mut sum:u32=0;
    let mut rem:u32;

    let count=no_of_digits(temp);

    while(num>0){
        rem=num%10;
        sum=sum+power(rem,count);
        num=num/10;
    }
    sum
}


fn no_of_digits(mut temp:u32)->u32{
    
    let mut count:u32=0;
    while(temp>0){
        temp=temp/10;
        count=count+1;
    }
    count
}


fn power(r:u32,mut c:u32)->u32{
    let mut pow:u32=1;
    while (c>0){
        pow=r*pow;
        c=c-1;
    }
    pow
}
