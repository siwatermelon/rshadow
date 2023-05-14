### rshadow
这是一个可以将文件（比如pe文件）隐藏在图片的小程序，使用rust编写。
新生成的图片仍然可以当作正常图片解析。

### 原理
原理很简单，有些图片格式规定了其可以保存一些元数据在图片中，这些数据不会被当作像素点，所以我们可以将需要隐藏的文件写进这些元数据中。

## 可能的应用场景
比如我们希望将我们的pe文件/一个恶意的dll文件伪装成一个图片。

## usage
Usage: rshadow hello.dll example.jpg

运行后我们的dll文件就写进了图片的元数据中，同时生成了一个buffer.dll.可以比较一下buffer.dll与hello.dll的文件大小，一致的话说明写入成功。example.jpg可以被正常解析。
