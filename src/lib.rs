mod basic;

use ndarray::prelude::*;

#[cfg(test)]
mod test{
    use ndarray::array;

    #[test]
    fn test(){
        let arr1 = array![[0, 1],
                                        [2, 3]];
        let arr2 = array![[4,5],
                                        [6,7]];
        println!("{}",arr1.clone() - arr2);
    }
}
