#[cfg(all(not(feature = "activate"), test))]
compile_error!("An original account transcript from IRS is necessary for this to work. If you have such a file, \
place it in the irs/ folder and turn on the `activate` feature of this crate.");

#[cfg(all(feature = "activate", test))]
mod test;
