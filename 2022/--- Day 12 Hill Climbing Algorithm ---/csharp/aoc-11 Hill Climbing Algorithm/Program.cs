// See https://aka.ms/new-console-template for more information

using System.Diagnostics;
using aoc_11_Hill_Climbing_Algorithm;

Console.WriteLine("Hello, World!");

var map = new AStarMap();

var lines = File.ReadAllText(@"..\..\..\test_input.txt");
map.FromString(lines);
//map.Init(5,5,new(0,0), new(4,4));

AStarMap.Result result;

Console.Clear();
Console.CursorVisible = false;
Stopwatch sw = new Stopwatch();
var pass = 0;
sw.Start();
while (true)
{
    result = map.Main();
    if (pass % 100 == 0 && result.lastSpot is not null && false)
    {
        map.ReconstructPath(result.lastSpot);
        Console.SetCursorPosition(0, 0);
        Console.Write($"{map.Visualize()}");
    }

    if (result.Finished)
    {
        sw.Stop();
        if (result is { Success: true, lastSpot: { } }) map.ReconstructPath(result.lastSpot);
        break;
    }

    pass++;
}

Console.SetCursorPosition(0, 0);

Console.Write($"{map.Visualize()}");
Console.WriteLine($"Done, steps taken: {map.EndSpot.G}, time: {sw.ElapsedMilliseconds}ms");

map.SolvePart2();