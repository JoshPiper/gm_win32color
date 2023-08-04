if not util.IsBinaryModuleInstalled("win32color") then
    print("Binary Module Not Installed")
    return
end

if not win32color then
    require("win32color")
end

print("MsgC Output")
local w, h = win32color.get_console_size()
for i = 1, w do
    MsgC(Color(math.Remap(i, 1, w, 0, 255), 0, 0), "|")
end
print()

for i = 1, w do
    MsgC(Color(0, math.Remap(i, 1, w, 0, 255), 0), "|")
end
print()

for i = 1, w do
    MsgC(Color(0, 0, math.Remap(i, 1, w, 0, 255)), "|")
end
print()

local from, to = ColorToHSV(Color(255, 0, 0)), ColorToHSV(Color(0, 255, 255))
for i = 1, w do
    MsgC(HSVToColor(Lerp(i / w, from, to), 1, 1), "|")
end
print()

print("ANSI Output")
win32color.test()
