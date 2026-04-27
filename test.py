from PIL import Image

img = Image.open("image-example1.jpeg").convert("RGB")
pixels = list(img.getdata())  # list of (R, G, B) tuples

# Try only Red channel
print("=== Only RED channel ===")
bits = []
for (r, g, b) in pixels:
    bits.append(r & 1)

msg = ""
for i in range(0, 200 * 8, 8):
    chunk = bits[i:i+8]
    byte = 0
    for b in chunk:
        byte = byte << 1 | b
    if 32 <= byte <= 126:
        msg += chr(byte)
    else:
        msg += f"[{byte}]"
print("MSB:", msg[:100])

# Try bit plane 1 (second LSB)
print("=== Bit plane 1 (second LSB) ===")
bits2 = []
for (r, g, b) in pixels:
    bits2.append((r >> 1) & 1)
    bits2.append((g >> 1) & 1)
    bits2.append((b >> 1) & 1)

msg2 = ""
for i in range(0, 200 * 8, 8):
    chunk = bits2[i:i+8]
    byte = 0
    for b in chunk:
        byte = byte << 1 | b
    if 32 <= byte <= 126:
        msg2 += chr(byte)
    else:
        msg2 += f"[{byte}]"
print("Bit1:", msg2[:100])

# Try all 8 bit planes
print("=== All bit planes ===")
for plane in range(8):
    bits3 = []
    for (r, g, b) in pixels:
        bits3.append((r >> plane) & 1)
        bits3.append((g >> plane) & 1)
        bits3.append((b >> plane) & 1)
    msg3 = ""
    for i in range(0, 200 * 8, 8):
        chunk = bits3[i:i+8]
        byte = 0
        for b in chunk:
            byte = byte << 1 | b
        if 32 <= byte <= 126:
            msg3 += chr(byte)
        else:
            msg3 += f"[{byte}]"
    print(f"Plane {plane}:", msg3[:80])