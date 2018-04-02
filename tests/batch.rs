extern crate serde_json;
extern crate cfn;

use cfn::{Resource, Expr};

#[test]
fn managed_ec2_batch_environment() {
    let json = include_str!("./fixtures/Managed_EC2_Batch_Environment.template");
    let tpl = serde_json::from_str::<cfn::Template>(json).unwrap();

    assert!(tpl.description().starts_with(
        "AWS CloudFormation Sample Template Managed Single Batch Job Queue"));

    let vpc = tpl.resources().get::<cfn::aws::ec2::VPC>("VPC").unwrap();
    assert_eq!(vpc.properties().cidr_block.as_value().unwrap(), "10.0.0.0/16");

    let _ = tpl.resources().get::<cfn::aws::ec2::InternetGateway>("InternetGateway").unwrap();

    let routes = tpl.resources().get::<cfn::aws::ec2::RouteTable>("RouteTable").unwrap();
    assert_eq!(routes.properties().vpc_id.as_reference().unwrap(), "VPC");

    let gateway_attachment = tpl.resources().get::<cfn::aws::ec2::VPCGatewayAttachment>("VPCGatewayAttachment").unwrap();
    assert_eq!(gateway_attachment.properties().vpc_id.as_reference().unwrap(), "VPC");
    assert_eq!(gateway_attachment.properties().internet_gateway_id.as_ref().unwrap().as_reference().unwrap(),
        "InternetGateway");

    let subnet = tpl.resources().get::<cfn::aws::ec2::Subnet>("Subnet").unwrap();
    assert_eq!(subnet.properties().cidr_block.as_value().unwrap(), "10.0.0.0/24");
    assert_eq!(subnet.properties().vpc_id.as_reference().unwrap(), "VPC");
    assert_eq!(subnet.properties().map_public_ip_on_launch.as_ref().unwrap().as_value().unwrap(), &true);

    let role = tpl.resources().get::<cfn::aws::iam::Role>("BatchServiceRole").unwrap();
    let arns = role.properties().managed_policy_arns.as_ref().unwrap().as_values().unwrap();
    assert_eq!(arns.len(), 1);
    assert_eq!(arns[0].as_value().unwrap(),
        "arn:aws:iam::aws:policy/service-role/AWSBatchServiceRole");

    let profile = tpl.resources().get::<cfn::aws::iam::InstanceProfile>("IamInstanceProfile").unwrap();
    let roles = profile.properties().roles.as_values().unwrap();
    assert_eq!(roles.len(), 1);
    assert_eq!(roles[0].as_reference().unwrap(), "EcsInstanceRole");

    let job_definition = tpl.resources().get::<cfn::aws::batch::JobDefinition>("JobDefinition").unwrap();
    assert_eq!(job_definition.properties().type_.as_value().unwrap(), "container");
    let container_props = job_definition.properties().container_properties.as_value().unwrap();
    assert_eq!(&2, container_props.vcpus.as_value().unwrap());
    assert_eq!(&2000, container_props.memory.as_value().unwrap());
    let image_expr = container_props.image.as_expression().unwrap();
    {
        let &Expr::Join { ref delimiter, ref values } = image_expr;
        assert_eq!("", delimiter);
        assert_eq!(3, values.len());
        assert_eq!("137112412989.dkr.ecr.", values[0].as_value().unwrap());
        assert_eq!("AWS::Region", values[1].as_reference().unwrap());
        assert_eq!(".amazonaws.com/amazonlinux:latest", values[2].as_value().unwrap());
    }

}
