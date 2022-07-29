use near_sdk::require;

use crate::*;

trait EduFormFunctions {
    fn add_form(&mut self, form: EduForm);
    fn remove_form(&mut self, form_id: u32);
    fn update_form(&mut self, form_id: u32, form: EduForm);
    fn get_forms(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<EduForm>;
    fn get_form_by_id(&self, form_id: u32) -> EduForm;
    fn remove_all_forms(&mut self);
    fn get_forms_count(&self) -> u64;
}

#[near_bindgen]
impl EduFormFunctions for Contract {
    fn add_form(&mut self, form: EduForm) {
        require!(self.admins.contains(&env::signer_account_id()), "Only admins can add forms");
        let len_forms = self.edu_forms.len().try_into().unwrap();
        self.edu_forms.insert(&len_forms, &form);
    }
    fn remove_form(&mut self, form_id: u32) {
        require!(self.admins.contains(&env::signer_account_id()), "Only admins can remove forms");
        self.edu_forms.remove(&form_id);
    }
    fn update_form(&mut self, form_id: u32, form: EduForm) {
        require!(self.admins.contains(&env::signer_account_id()), "Only admins can update forms");
        self.edu_forms.insert(&form_id, &form);
    }
    fn get_forms(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<EduForm> {
        let forms = self.edu_forms.keys_as_vector();
        let start: u128 = u128::from(from_index.unwrap_or(U128(0)));
        forms.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|key| self.edu_forms.get(&key).unwrap())
            .collect()
    }
    fn get_form_by_id(&self, form_id: u32) -> EduForm {
        self.edu_forms.get(&form_id).unwrap().clone()
    }
    fn remove_all_forms(&mut self) {
        require!(self.owner == env::signer_account_id(), "Only owner can clear forms");
        self.edu_forms.clear();
    }
    fn get_forms_count(&self) -> u64 {
        self.edu_forms.len() as u64
    }
}
