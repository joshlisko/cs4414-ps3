#/bin/sh



URL="192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt 192.168.0.188:4414/advance.docx 192.168.0.188:4414/test.txt"

echo $URL | xargs -n 1 -P 8 wget
