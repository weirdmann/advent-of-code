// See https://aka.ms/new-console-template for more information

Console.WriteLine("Hello, World!");

var firstPass = Packet.Element.ParseFromString("[1,2,3,4]");

Packet.Element.ParseFromString("1,2,3,4");

var e1 = Packet.Element.FromString("[]");
var e2 = Packet.Element.FromString("1");
var e3 = Packet.Element.FromString("[1,2,3,4]");
var e4 = Packet.Element.FromString("[1,[2,3],4]");

Console.WriteLine();