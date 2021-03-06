## 利用Rust实现TIME协议
## 设计要求
TIME协议是基于TCP/IP的应用层协议,如果一台PC机希望精确地设置自己的日期和时间,它可以访问网络上运行 TIME SERVER程序的服务器,可参阅RFC868[6-5].
本次设计要求利用Java实现TIME协议的基本功能.
## 相关内容
* 端口:37
* 协议:TCP/UDP
* 用途:时间服务器返回服务器从1900年1月1日子夜过去后的秒数,这是一个4字节有符号big-endian整数
* 知识点
  * [网络传输大小端](https://blog.csdn.net/weixin_40292830/article/details/95618771)
  * [UTC时间与北京时间换算](https://datetime360.com/cn/utc-beijing-time/)
## 参考书目
* 基于Windows的TCP/IP编程
* The Rust Programming Language
* The Cargo Book