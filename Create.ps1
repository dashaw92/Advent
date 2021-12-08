if($args.Count -le 1) {
    Write-Host "Usage: ./Create.ps1 <year> <day>"
    return
}

$year=$args[0]
$day=$args[1]

if(-not (Test-Path -Path "$year")) {
    Write-Host "Year $year does not exist yet."
    if($args.Count -ge 3 -and $args[2] -eq "force") {
        Write-Host "...but force flag was found. Creating!"
        mkdir "$year"
    } else {
        return
    }
}

if(Test-Path "$year\day$day") {
    Write-Host "$year\day$day project already exists!"
    return
}

Set-Location "$year"
Write-Host "Creating day$day cargo --bin project..."
cargo new --bin ("day$day")
Set-Location ..
Write-Host "Writing template to main.rs"
Get-Content "template.rs" > "$year\day$day\src\main.rs"
Set-Location "$year\day$day"
Write-Host "Done."