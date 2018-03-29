extern crate serde_json;
extern crate cfn;

use cfn::Resource;

#[test]
fn managed_ec2_batch_environment() {
    let json = include_str!("./fixtures/Managed_EC2_Batch_Environment.template");
    let tpl = serde_json::from_str::<cfn::Template>(json).unwrap();

    let vpc = tpl.resources().get::<cfn::aws::ec2::VPC>("VPC").unwrap();
    assert_eq!(vpc.properties().cidr_block, "10.0.0.0/16");

    let _ = tpl.resources().get::<cfn::aws::ec2::InternetGateway>("InternetGateway").unwrap();

    let role = tpl.resources().get::<cfn::aws::iam::Role>("BatchServiceRole").unwrap();
    let arns = role.properties().managed_policy_arns.as_ref().unwrap();
    assert_eq!(arns.len(), 1);
    assert_eq!(arns[0], "arn:aws:iam::aws:policy/service-role/AWSBatchServiceRole");
}
