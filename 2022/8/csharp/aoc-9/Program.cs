// See https://aka.ms/new-console-template for more information
// https://adventofcode.com/2022/day/8/input
//
using System.Collections;
using System.Linq.Expressions;
using System.Runtime.ExceptionServices;

Console.WriteLine("Hello, World!");
var lines = File.ReadAllLines(@"..\..\..\puzzle_input.txt");

uint[,] rows = new uint[lines.Length, lines[0].Length];
uint[,] cols = new uint[lines[0].Length, lines.Length];

Tree[,] treeArray = new Tree[lines.Length, lines.Length];


List<Tree> trees = new List<Tree>();


for (int i = 0; i < lines.Length; i++)
// rows from top to bottom
{
    for (int j = 0; j < lines[i].Length; j++)
    // columns from left to right
    {
        rows[i, j] = (uint)lines[i].ToCharArray()[j] - 0x30;
        cols[i, j] = (uint)lines[j].ToCharArray()[i] - 0x30;

        Tree newTree = new Tree() { Height = rows[i, j] };
        treeArray[i, j] = newTree;
    }
}

for (int i = 0; i < treeArray.GetLength(0); i++)
{
    for (int j = 0; j < treeArray.GetLength(1); j++)
    {
        if (treeArray[i, j] == null) continue;

        if (i > 0)
            treeArray[i, j].Up = treeArray[i - 1, j];

        if (i < treeArray.GetLength(0) - 1)
            treeArray[i, j].Down = treeArray[i + 1, j];

        if (j > 0)
            treeArray[i, j].Left = treeArray[i, j - 1];

        if (j < treeArray.GetLength(1) - 1)
            treeArray[i, j].Right = treeArray[i, j + 1];
    }
}

var visible = 0;
uint scenic_score = 0;
foreach (var tree in treeArray)
{
    if (tree is null) continue;
    if (tree.isVisible)
    {
        visible++;
    }
    scenic_score = Math.Max(tree.ScenicScore, scenic_score);
}

Console.WriteLine($"Part 1: {visible}\nPart 2: {scenic_score}");




class Tree
{
    public uint Height;
    public Tree Up;
    public Tree Down;
    public Tree Left;
    public Tree Right;


    public uint maxUp
    {
        get
        {
            if (Up is null) return 0;
            return Math.Max(Up.maxUp, Up.Height);
        }
    }
    public uint maxDown
    {
        get
        {
            if (Down is null) return 0;
            return Math.Max(Down.maxDown, Down.Height);
        }
    }
    public uint maxLeft
    {
        get
        {
            if (Left is null) return 0;
            return Math.Max(Left.maxLeft, Left.Height);
        }
    }
    public uint maxRight
    {
        get
        {
            if (Right is null) return 0;
            return Math.Max(Right.maxRight, Right.Height);
        }
    }

    public bool isVisibleUp
    {
        get
        {
            return Height > maxUp | Up is null;
        }
    }

    public bool isVisibleDown
    {
        get
        {
            return Height > maxDown | Down is null;
        }
    }

    public bool isVisibleLeft
    {
        get
        {
            return Height > maxLeft | Left is null;
        }
    }

    public bool isVisibleRight
    {
        get
        {
            return Height > maxRight | Right is null;
        }
    }

    public bool isVisible
    {
        get
        {
            return isVisibleUp | isVisibleDown | isVisibleLeft | isVisibleRight;
        }
    }

    public uint ScenicScore
    {
        get
        {
            uint a = 0;
            uint b = 0;
            uint c = 0;
            uint d = 0;
            return CountViewingDistanceUp(Height, ref a)
                * CountViewingDistanceDown(Height, ref b)
                * CountViewingDistanceLeft(Height, ref c)
                * CountViewingDistanceRight(Height, ref d);

        }
    }

    public uint CountViewingDistanceUp(uint h, ref uint c)
    {

        if (Up is null) return c;
        c++;
        if (Up.Height >= h) return c;
        return Up.CountViewingDistanceUp(h, ref c);
    }

    public uint CountViewingDistanceDown(uint h, ref uint c)
    {

        if (Down is null) return c;
        c++;
        if (Down.Height >= h) return c;
        return Down.CountViewingDistanceDown(h, ref c);
    }

    public uint CountViewingDistanceLeft(uint h, ref uint c)
    {
        if (Left is null) return c;
        c++;
        if (Left.Height >= h) return c;
        return Left.CountViewingDistanceLeft(h, ref c);
    }

    public uint CountViewingDistanceRight(uint h, ref uint c)
    {
        if (Right is null) return c;
        c++;
        if (Right.Height >= h) return c;
        return Right.CountViewingDistanceRight(h, ref c);
    }
}