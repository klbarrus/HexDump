[CmdletBinding()]

param(
    [string] $filePath = ""
    )

$offset = 0

Get-Content -Encoding Byte $filePath -ReadCount 16 | `

ForEach-Object {
    $hex = ""
    $ascii = ""
    foreach ($byte in $_) {
        $hex += "{0:X2} " -f $byte
        $byteAsChar = [convert]::ToChar($byte)
        if ([char]::IsControl($byte)) {
            $byteAsChar = '.'
        }
        $ascii += "{0} " -f $byteAsChar
    }
    $line = "{0:X8} " -f $offset
    $line + $hex + "- " + $ascii
    $offset += 16
}