// See https://aka.ms/new-console-template for more information

using aoc_7;


var home = new ElFileSystem.Directory("", null);

home.mkdir("papaj1").OnSuccess((d) =>
{
    d.AddFile(new ElFileSystem.File("file1", 1));
    d.AddFile(new ElFileSystem.File("file2", 2));
    d.AddFile(new ElFileSystem.File("file3", 3));
    d.AddFile(new ElFileSystem.File("file4", 4));
    d.AddFile(new ElFileSystem.File("file5", 5));
    d.AddFile(new ElFileSystem.File("file6", 6));
}
);
home.mkdir("papaj1").OnFail((d) =>
{
    Console.WriteLine("Adding failed: {0}", d.Name);
    home.mkdir("papaj2");
});

home.AddFile(new ElFileSystem.File("huj", 1234567));

Console.WriteLine(home);