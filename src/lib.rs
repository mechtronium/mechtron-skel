use wasm_membrane_guest::membrane::log;
use mechtron::{Mechtron, mechtron_register, MechtronCtx, MechtronFactory};
use mechtron::error::Error;
use mesh_portal::version::latest::entity::request::Action;
use mesh_portal::version::latest::entity::response::ResponseCore;
use mesh_portal::version::latest::http::HttpRequest;
use mesh_portal::version::latest::resource::ResourceStub;


/// mechtron_init() is called after the Wasm file has been compiled by Starlane.
/// It's most important job is to register any mechtron factories.
#[no_mangle]
pub extern "C" fn mechtron_init()
{
    /// Here we register the MyMechtronFactory for creating 'my-mechtron' Mechtrons
    mechtron_register(Box::new(MyMechtronFactory::new()));
}

/// Factory implementation for MyMechtron Mechtron
pub struct MyMechtronFactory { }

impl MyMechtronFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl MechtronFactory for MyMechtronFactory {

    /// Here we returning the very important Mechtron name which will must be referenced in the Mechtron config
    fn mechtron_name(&self) -> String {
        "my-mechtron".to_string()
    }

    fn create(&self, stub: ResourceStub) -> Result<Box<dyn Mechtron>, Error> {
        Ok(Box::new(MyMechtron::new()))
    }
}


/// MyMechtron mechtron is an implementation of the Mechtron trait.
/// It handles requests and produces Responses
pub struct MyMechtron {}

impl MyMechtron {
    pub fn new()->Self{
        Self{}
    }
}

impl Mechtron for MyMechtron  {

    /// Write custom Msg request handler code here
    fn handle_msg_request(
        &self,
        ctx: &dyn MechtronCtx,
        request: MsgRequest,
    ) -> Result<ResponseCore, Error> {
        Ok(request.fail(format!(
            "Mechtron '{}' does not have a Msg handler implementation",
            ctx.stub().address.to_string()
        ).as_str()))
    }

    /// Write custom Http request handler code here
    fn handle_http_request(
        &self,
        ctx: &dyn MechtronCtx,
        request: HttpRequest,
    ) -> Result<ResponseCore, Error> {
        Ok(request.fail(format!(
            "Mechtron '{}' does not have an Http handler implementation",
            ctx.stub().address.to_string()
        ).as_str()))
    }

}

