impl UntypedActionSupport {
    pub fn new_from(typename: &str) -> Result<Self> {
        #[allow(non_snake_case)]
        fn new_untyped_service_support_action_tutorials_interfaces_action_Fibonacci() -> UntypedActionSupport {
            UntypedActionSupport::new::<
                action_tutorials_interfaces::action::Fibonacci::Action,
            >()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_example_interfaces_action_Fibonacci() -> UntypedActionSupport {
            UntypedActionSupport::new::<example_interfaces::action::Fibonacci::Action>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_tf2_msgs_action_LookupTransform() -> UntypedActionSupport {
            UntypedActionSupport::new::<tf2_msgs::action::LookupTransform::Action>()
        }
        #[allow(non_snake_case)]
        fn new_untyped_service_support_turtlesim_action_RotateAbsolute() -> UntypedActionSupport {
            UntypedActionSupport::new::<turtlesim::action::RotateAbsolute::Action>()
        }
        static MAP: phf::Map<&'static str, fn() -> UntypedActionSupport> = phf::phf_map! {
            "action_tutorials_interfaces/action/Fibonacci" =>
            new_untyped_service_support_action_tutorials_interfaces_action_Fibonacci,
            "example_interfaces/action/Fibonacci" =>
            new_untyped_service_support_example_interfaces_action_Fibonacci,
            "tf2_msgs/action/LookupTransform" =>
            new_untyped_service_support_tf2_msgs_action_LookupTransform,
            "turtlesim/action/RotateAbsolute" =>
            new_untyped_service_support_turtlesim_action_RotateAbsolute
        };
        let func = MAP
            .get(typename)
            .ok_or_else(|| Error::InvalidMessageType {
                msgtype: typename.into(),
            })?;
        Ok(func())
    }
}
