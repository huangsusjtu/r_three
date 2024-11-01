import os

def count_lines_in_directory(directory):
   total_lines = 0
   for root, dirs, files in os.walk(directory):
       for file in files:
           file_extension = os.path.splitext(file)[1]
           if file_extension in ['.cpp', '.h', '.java', '.py', '.rs']:
               file_path = os.path.join(root, file)
               try:
                   with open(file_path, 'r') as f:
                       lines = f.readlines()
                       total_lines += len(lines)
               except UnicodeDecodeError:
                   print(f"Could not read {file_path}")
   return total_lines


if __name__ == "__main__":
   directory = "C:\\Users\\huangsu\\RustroverProjects\\r-three\\src"
   line_count = count_lines_in_directory(directory)
   print(f"Total lines of code in {directory}: {line_count}")