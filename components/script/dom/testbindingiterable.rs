/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// check-tidy: no specs after this line

use dom_struct::dom_struct;
use js::rust::HandleObject;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::TestBindingIterableBinding::TestBindingIterableMethods;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::reflector::{Reflector, reflect_dom_object_with_proto};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct TestBindingIterable {
    reflector: Reflector,
    vals: DomRefCell<Vec<DOMString>>,
}

impl TestBindingIterable {
    fn new(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
    ) -> DomRoot<TestBindingIterable> {
        reflect_dom_object_with_proto(
            Box::new(TestBindingIterable {
                reflector: Reflector::new(),
                vals: DomRefCell::new(vec![]),
            }),
            global,
            proto,
            can_gc,
        )
    }
}

impl TestBindingIterableMethods<crate::DomTypeHolder> for TestBindingIterable {
    fn Constructor(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
    ) -> Fallible<DomRoot<TestBindingIterable>> {
        Ok(TestBindingIterable::new(global, proto, can_gc))
    }

    fn Add(&self, v: DOMString) {
        self.vals.borrow_mut().push(v);
    }
    fn Length(&self) -> u32 {
        self.vals.borrow().len() as u32
    }
    fn GetItem(&self, n: u32) -> DOMString {
        self.IndexedGetter(n).unwrap_or_default()
    }
    fn IndexedGetter(&self, n: u32) -> Option<DOMString> {
        self.vals.borrow().get(n as usize).cloned()
    }
}
