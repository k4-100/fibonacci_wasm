
/// returns fibonacci sequence in form of a string
pub fn get_fibonacci_string( count : usize ) -> String {
    let mut seq : Vec<u32> = vec![0,1];
    let mut f_str: String = String::new();
    f_str.push_str(
        format!("0: {}\n1: {}\n", seq[0],seq[1]).as_str()
    );

    for x in 2..2+count{
        seq.push( seq[x-2] + seq[x-1] );
        f_str.push_str( 
            format!("{}: {}\n" ,x,seq[x]).as_str()
        );
    }

    return f_str;
}