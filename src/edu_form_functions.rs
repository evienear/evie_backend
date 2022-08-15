use near_sdk::{require, collections::Vector};

use crate::*;

trait EduFormFunctions {
    fn add_form(&mut self, form: EduForm);
    fn remove_form(&mut self, form_id: u32);
    fn update_form(&mut self, form_id: u32, form: EduForm);
    fn get_forms(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<EduFormWithId>;
    fn get_form_by_id(&self, form_id: u32) -> EduFormWithId;
    fn remove_all_forms(&mut self);
    fn get_forms_count(&self) -> u64;
}

#[near_bindgen]
impl EduFormFunctions for Contract {
    fn add_form(&mut self, form: EduForm) {
        require!((self.admins.contains(&env::signer_account_id() ) || self.owner == env::signer_account_id()), "Only admins or the owner can add forms");
        let form_id: u32 = self.edu_form_number;
        self.edu_forms.insert(&form_id, &form);
        self.edu_form_number += 1;
    }
    fn remove_form(&mut self, form_id: u32) {
        require!((self.admins.contains(&env::signer_account_id() ) || self.owner == env::signer_account_id()), "Only admins or the owner can remove forms");
        self.edu_forms.remove(&form_id);
    }
    fn update_form(&mut self, form_id: u32, form: EduForm) {
        require!((self.admins.contains(&env::signer_account_id() ) || self.owner == env::signer_account_id()), "Only admins or the owner can update forms");
        self.edu_forms.insert(&form_id, &form);
    }
    fn get_forms(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<EduFormWithId> {
        // let forms: &Vector<u32> = self.edu_forms.keys_as_vector();
        let start: u128 = u128::from(from_index.unwrap_or(U128(0)));
        let keys: Vec<u32> = self.edu_forms.keys_as_vector().to_vec();
        
        let forms: Vec<EduFormWithId> = keys.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|key: &u32| {
                let form = self.edu_forms.get(key).unwrap();
                EduFormWithId {
                    id: key.clone(),
                    form: form.clone(),
                }
            })
            .collect();
        forms
    }
    fn get_form_by_id(&self, form_id: u32) -> EduFormWithId {
        let pre_edu_form = self.edu_forms.get(&form_id).unwrap().clone();
        let form = EduFormWithId {
            id: form_id,
            form: pre_edu_form,
        };
        form
    }
    fn remove_all_forms(&mut self) {
        require!(self.owner == env::signer_account_id(), "Only owner can clear forms");
        self.edu_forms.clear();
    }
    fn get_forms_count(&self) -> u64 {
        self.edu_forms.len() as u64
    }
}
