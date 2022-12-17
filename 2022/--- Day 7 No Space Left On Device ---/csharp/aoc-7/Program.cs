// See https://aka.ms/new-console-template for more information

using aoc_7;

var home = new ElFileSystem.Directory("", null);


var commands = ElFileSystem.ParsedCommand.ParseFile(File.ReadAllText(@"..\..\..\puzzle_input.txt"));
ElFileSystem.Directory cwd = home;
ElFileSystem.Directory last;
var first = true;
var i = 0;


foreach (var command in commands)
{
    i++;
    if (command.cd_to.Equals(".."))
    {
        last = cwd;
        cwd = cwd.parent;
        first = true;
    };

    if (cwd is null) throw new Exception("parent is null");

    if (!first)
    {
        cwd.mkdir(command.cd_to);
        cwd = cwd.Subdirectories.GetValueOrDefault(command.cd_to);
    };
    first = false;

    if (cwd is null) throw new Exception("cwd is null");



    if (cwd == null) throw new Exception("error switching directories");

    foreach (var dir in command.directories)
    {
        cwd.mkdir(dir);
    }
    foreach (var file in command.files)
    {
        cwd.AddFile(new ElFileSystem.File(file.Key, file.Value));
    }
}

var answer = new PuzzleAnswer(home);

Console.WriteLine(answer);



struct PuzzleAnswer
{
    public ulong answer_part_1 = 0;
    public ulong answer_part_2 = 0;
    public ulong totalDiskSpace = 70000000;
    public ulong requiredDiskSpace = 30000000;
    public ulong freeDiskSpace = 0;
    ElFileSystem.Directory home;
    public PuzzleAnswer(ElFileSystem.Directory home)
    {
        this.home = home;
        this.freeDiskSpace = this.totalDiskSpace - home.size_recursive;
        ScanDirectoriesForPart1(home);
        this.answer_part_2 = totalDiskSpace;
        ScanDirectoriesForPart2(home);
    }

    public void ScanDirectoriesForPart1(ElFileSystem.Directory directory)
    {
        if (directory.size_recursive < 100000) answer_part_1 += directory.size_recursive;
        foreach (var d in directory.Subdirectories)
        {
            ScanDirectoriesForPart1(d.Value);
        }
    }

    public void ScanDirectoriesForPart2(ElFileSystem.Directory directory)
    {
        if (directory.size_recursive >= (requiredDiskSpace - freeDiskSpace)) answer_part_2 = Math.Min(answer_part_2, directory.size_recursive);
        foreach (var d in directory.Subdirectories)
        {
            ScanDirectoriesForPart2(d.Value);
        }
    }

    public override string ToString()
    {
        return $"\n" +
            $"Puzzle Answer 1: {answer_part_1}\n" +
            $"Free disk space: {freeDiskSpace}/{totalDiskSpace}\n" +
            $"Puzzle Answer 2: {answer_part_2}";
    }

}