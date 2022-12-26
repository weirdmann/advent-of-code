// See https://aka.ms/new-console-template for more information

using System.Collections;

Console.WriteLine("Hello, World!");

var a = new List<int>(){ 1, 2, 3 };
var b = new List<int>(){ 1, 2, 3 };

var firstPass = Packet.ParseSinglePacket("[[[[],1,2],[[7,7,7],6,[1,7,1]]],[4,8],[]]");
var split = firstPass.Split(Environment.NewLine);
Packet.ParseSinglePacket(split[1]);

Packet.ParseSinglePacket("1,2,3,4");