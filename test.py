from PIL import Image

img = Image.open("image-example1.jpeg").convert("RGB")
pixels = list(img.getdata())

bits = []
for (r, g, b) in pixels:
    if r > 127:
        bits.append(1)
    else:
        bits.append(0)

msg = ""
for i in range(0, len(bits) - 8, 8):
    chunk = bits[i:i+8]
    byte = 0
    for b in chunk:
        byte = byte << 1 | b
    if 32 <= byte <= 126 or byte in (10, 13):
        msg += chr(byte)
    else:
        msg += f"[{byte}]"
    if "-----END PGP PUBLIC KEY BLOCK-----" in msg:
        break

print(msg[:500])