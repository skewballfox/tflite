import tflite as tf
from pathlib import Path

file_bytes=Path("test_data/hey_mycroft.tflite").read_bytes()
print(tf.flatbuffers.encode.Get(tf.flatbuffers.packer.uoffset, file_bytes,0))
model=tf.Model.GetRootAs(file_bytes,0)
print(model.Description())

print(tf.flatbuffers.packer.uoffset.unpack_from(tf.flatbuffers.compat.memoryview_type(file_bytes),0))