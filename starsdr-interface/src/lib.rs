use async_trait::async_trait;


#[async_trait]
pub trait Driver: Send {
   async fn __dyn(&self) where Self: Sized {}
   async fn list(&self)->Vec<Box<dyn SDRDevice>>; 
}



pub trait SDRDevice: Send {
}