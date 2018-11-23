use traits::base::NormalForm;
use utils::ZiaResult;

pub trait NormalFormModifier
where
    Self: NormalForm<Self> + Clone,
{
    fn update_normal_form(&mut self, normal_form: &mut Self) -> ZiaResult<()> {
        try!(self.set_normal_form(normal_form));
        normal_form.add_reduces_from(self);
        Ok(())
    }
    fn delete_normal_form(&mut self) -> ZiaResult<()> {
        match try!(self.get_normal_form()) {
            None => (),
            Some(mut n) => {
                n.remove_reduces_from(self);
                self.remove_normal_form();
            }
        };
        Ok(())
    }
}
