import os

def count_code_lines(file_path):
    """统计单个代码文件的代码行数"""
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            lines = file.readlines()
            # 去掉空行和注释行
            code_lines = [line for line in lines if line.strip() and not line.strip().startswith('#')]
            return len(code_lines)
    except Exception as e:
        print(f"无法读取文件 {file_path}: {e}")
        return 0

def traverse_and_count_code_lines(directory):
    """遍历指定目录及子目录，统计所有代码文件的行数"""
    total_lines = 0
    file_count = 0

    # 遍历目录
    for root, dirs, files in os.walk(directory):
        for file in files:
            # 只处理常见的代码文件类型
            if file.endswith(('.py', '.js', '.java', '.cpp', '.c', '.html', '.css', '.go','.rs')):
                file_path = os.path.join(root, file)
                lines_in_file = count_code_lines(file_path)
                if lines_in_file > 0:
                    print(f"{file_path}: {lines_in_file} 行代码")
                    total_lines += lines_in_file
                    file_count += 1

    print(f"\n总共有 {file_count} 个代码文件，总行数为 {total_lines} 行。")

# 使用示例
directory_to_scan = os.getcwd()  # 替换为你要扫描的目录
traverse_and_count_code_lines(directory_to_scan)

