# auto_deploy_dockers
docker接口客户端

主要实现接口
| 接口名称  | 接口 |   请求方式 |
| :------------- | :----------: | ------------: |
| docker版本信息 | /                | GET |
| docker镜像列表 | /images          | GET |
| docker容器列表 | /containers      | GET |
| docker创建镜像 | /build           | POST|
| docker创建容器 | /start           | POST|
| docker进入容器 | /exec            | POST|
| docker容器状态 | /state           | GET |
