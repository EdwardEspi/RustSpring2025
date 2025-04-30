fn main() {

    // https://docs.rs/rayon/latest/rayon/
    extern crate rayon; // 1.5.3
    use rayon::prelude::*; // 1.5.3
    
    let wiki_txt = " Parallel computing is a type of computation in which many calculations or processes are carried out simultaneously.
    Large problems can often be divided into smaller ones, which can then be solved at the same time. 
    There are several different forms of parallel computing: bit-level, instruction-level, data, and task parallelism. 
    Parallelism has long been employed in high-performance computing, but has gained broader interest due to the physical 
    constraints preventing frequency scaling.Parallel computing is closely related to concurrent computing—
    they are frequently used together, and often conflated, though the two are distinct: 
    it is possible to have parallelism without concurrency, and concurrency without parallelism 
    (such as multitasking by time-sharing on a single-core CPU).
    In parallel computing, a computational task is typically broken down into several, often many, 
    very similar sub-tasks that can be processed independently and whose results are combined afterwards, upon completion. 
    In contrast, in concurrent computing, the various processes often do not address related tasks; 
    when they do, as is typical in distributed computing, the separate tasks may have a varied nature and often require some 
    inter-process communication during execution.";
    
    let words:Vec<_> = wiki_txt.split_whitespace().collect();
    
    // par_iter() -> parallel iterator
    
    words.par_iter().for_each(|val| println!("{}",val));
    
    // par_iterator can do everything as regular iterator, but can does it 
    // in parallel
    
    let words_with_p: Vec<_> = words
        .par_iter()
        .filter(|val| val.find('p').is_some()) // of course notice the closure FN, which borrows for reading only
        .collect();
        
        
    println!("All words with letter p: {:?}",words_with_p);
    
}
