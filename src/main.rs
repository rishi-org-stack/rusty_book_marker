// mod hello;
// use hello::{pk};
// fn parse(s: &str)
// {
//     if s.len()==0{
//         return
//     }
//     for (i,&item) in s.as_bytes().iter().enumerate(){
//         if item==b' '{
//             let x=&s[0..i+1];
//             println!("{}",x);

//         }
//     }

// }

struct Query<'a> {
    command: &'a str,
    loc: &'a str,
}



fn parse_2<'a>(q : &'a mut Query<'a>,s: &'a str, start: usize, end: usize)-> &'a mut Query<'a>{
    if end >= s.len() {
        if start == 0 {
          q.command = &s[start..end];
        } else {
            q.loc = &s[start..end];
        }  
        return q;
    }
    if s.chars().nth(end).unwrap() == ' ' && s.chars().nth(start).unwrap() != ' ' {
        if start == 0 {
            q.command = &s[start..end + 1];
        }else {
            q.loc = &s[start..end + 1]
        }
        parse_2(q,s, end + 1, end + 1)
    } else if s.chars().nth(end).unwrap() == ' ' && s.chars().nth(start).unwrap() == ' ' {
        parse_2(q,s, start + 1, end + 1)
    }else {
       
        parse_2(q,s, start, end + 1, )
    }
}
fn main() {
    // println!("Hello, world!");
    // println!("{}",pk());\
    let q = &mut Query {
        command: "",
        loc: "",
    };
    let x=parse_2(q,"ok jha okkkk", 0, 0);
    println!("command {}",x.command);
    println!("command {}",x.loc)
}
