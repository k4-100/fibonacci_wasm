
/// returns fibonacci sequence in form of a string
pub fn get_fibonacci_string( count : usize ) -> String {
    let mut seq : Vec<u32> = vec![0,1];
    let mut f_str: String = String::new();
    
    for x in 2..count{
        seq.push( seq[x-2] + seq[x-1] );
    }
    
    seq.iter().for_each(
        |n| f_str.push_str( format!("{}\n", n).as_str() )
    );

    return f_str;
}