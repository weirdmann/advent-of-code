// See https://aka.ms/new-console-template for more information

using aoc_11_Hill_Climbing_Algorithm;

Console.WriteLine("Hello, World!");

var map = new AStarMap();

var lines = File.ReadAllText(@"..\..\..\test_input.txt");
map.FromString(lines);
//map.Init(5,5,new(0,0), new(4,4));

AStarMap.Result result;

Console.Clear();
Console.CursorVisible = false;

while (true)
{
    result = map.Main();

     //Console.SetCursorPosition(0,0);
     //Console.Write($"{map.Visualize()}");
    if (result.Finished)
    {
        break;
    }
}
Console.SetCursorPosition(0,0);
Console.Write($"{map.Visualize()}");
Console.WriteLine($"Done, steps taken: {map.EndSpot.G}, {result.Path?.Count}");
