// Take a look at the license at the top of the repository in the LICENSE file.

use gtk::prelude::*;
use gtk::glib;

mod imp {
    use super::*;
    use gtk::subclass::prelude::*;
    use gtk::CompositeTemplate;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(string = r#"
    <interface>
       <template class="MyWidget" parent="GtkWidget">
          <child>
            <object class="GtkLabel" id="the_label"/>
          </child>
       </template>
    </interface>
    "#)]
    pub struct MyWidget {
        #[template_child(id = "the_label", id = "the_label")]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MyWidget {
        const NAME: &'static str = "MyWidget";
        type Type = super::MyWidget;
        type ParentType = gtk::Widget;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MyWidget {
        fn dispose(&self, obj: &Self::Type) {
            if let Some(child) = obj.first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for MyWidget {}
}

glib::wrapper! {
    pub struct MyWidget(ObjectSubclass<imp::MyWidget>) @extends gtk::Widget;
}

fn main() {}
