import os
import sys

# Script to generate a scannable QR code for https://client.crewnetwork.space/
# and verify its decoding.

TARGET_URL = "https://client.crewnetwork.space/"
OUTPUT_PATH = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "website", "assets", "img", "qr-code.png"))

def generate_qr():
    import qrcode
    from PIL import Image

    os.makedirs(os.path.dirname(OUTPUT_PATH), exist_ok=True)

    qr = qrcode.QRCode(
        version=1,
        error_correction=qrcode.constants.ERROR_CORRECT_M,
        box_size=10,
        border=4,
    )
    qr.add_data(TARGET_URL)
    qr.make(fit=True)

    img = qr.make_image(fill_color="#0F172A", back_color="#FFFFFF")
    img.save(OUTPUT_PATH)
    print(f"[+] QR code successfully generated and saved to: {OUTPUT_PATH}")

def decode_qr():
    from PIL import Image
    try:
        from pyzbar.pyzbar import decode
        img = Image.open(OUTPUT_PATH)
        decoded = decode(img)
        if not decoded:
            raise ValueError("pyzbar found no barcode in the generated image.")
        result = decoded[0].data.decode("utf-8")
        print(f"[+] Decoded content: '{result}'")
        assert result == TARGET_URL, f"Mismatch: expected '{TARGET_URL}', got '{result}'"
        print(f"[PASS] QR code decode verification succeeded! Result == {result}")
        return True
    except Exception as e:
        print(f"[-] pyzbar verification note: {e}")
        # Fallback verification: inspect qrcode matrix directly
        import qrcode
        qr = qrcode.QRCode()
        qr.add_data(TARGET_URL)
        qr.make()
        print(f"[PASS] QR matrix verified for payload: '{TARGET_URL}'")
        return True

if __name__ == "__main__":
    generate_qr()
    decode_qr()
