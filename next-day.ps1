$day = Read-Host -Prompt "Day";
$name = Read-Host -Prompt "Name";

$dayParsed = 0;
if (-not ([int]::TryParse($day, [ref] $dayParsed))) {
    throw "Invalid day";
}

$projectName = "day";

if ($dayParsed -lt 10) { 
    $projectName += "0";
}
$projectName += $dayParsed
$projectName += "_{0}" -f $name


&cargo new $projectName

Copy-Item "lib/*" ("{0}/src" -f $projectName )
