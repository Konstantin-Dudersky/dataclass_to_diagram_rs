use std::rc::Rc;

pub fn clone_vec_of_ref_rc<T>(input: &Vec<&Rc<T>>) -> Vec<Rc<T>> {
    input.iter().map(|t| (*t).clone()).collect()
}

pub fn clone_vec_of_rc<T>(input: &Vec<Rc<T>>) -> Vec<Rc<T>> {
    input.iter().map(|t| (*t).clone()).collect()
}
