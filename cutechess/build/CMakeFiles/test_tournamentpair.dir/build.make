# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.28

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build

# Include any dependencies generated for this target.
include CMakeFiles/test_tournamentpair.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/test_tournamentpair.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/test_tournamentpair.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/test_tournamentpair.dir/flags.make

test_tournamentpair_autogen/timestamp: /usr/lib/qt5/bin/moc
test_tournamentpair_autogen/timestamp: /usr/lib/qt5/bin/uic
test_tournamentpair_autogen/timestamp: CMakeFiles/test_tournamentpair.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --blue --bold --progress-dir=/home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Automatic MOC and UIC for target test_tournamentpair"
	/usr/bin/cmake -E cmake_autogen /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build/CMakeFiles/test_tournamentpair_autogen.dir/AutogenInfo.json RelWithDebInfo
	/usr/bin/cmake -E touch /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build/test_tournamentpair_autogen/timestamp

CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.o: CMakeFiles/test_tournamentpair.dir/flags.make
CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.o: test_tournamentpair_autogen/mocs_compilation.cpp
CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.o: CMakeFiles/test_tournamentpair.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.o -MF CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.o.d -o CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.o -c /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build/test_tournamentpair_autogen/mocs_compilation.cpp

CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build/test_tournamentpair_autogen/mocs_compilation.cpp > CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.i

CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build/test_tournamentpair_autogen/mocs_compilation.cpp -o CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.s

CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.o: CMakeFiles/test_tournamentpair.dir/flags.make
CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.o: /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp
CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.o: CMakeFiles/test_tournamentpair.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building CXX object CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.o -MF CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.o.d -o CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.o -c /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp

CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp > CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.i

CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp -o CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.s

# Object files for target test_tournamentpair
test_tournamentpair_OBJECTS = \
"CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.o" \
"CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.o"

# External object files for target test_tournamentpair
test_tournamentpair_EXTERNAL_OBJECTS =

test_tournamentpair: CMakeFiles/test_tournamentpair.dir/test_tournamentpair_autogen/mocs_compilation.cpp.o
test_tournamentpair: CMakeFiles/test_tournamentpair.dir/projects/lib/tests/tournamentpair/tst_tournamentpair.cpp.o
test_tournamentpair: CMakeFiles/test_tournamentpair.dir/build.make
test_tournamentpair: libcutechess.a
test_tournamentpair: /usr/lib/x86_64-linux-gnu/libQt5Concurrent.so.5.15.13
test_tournamentpair: /usr/lib/x86_64-linux-gnu/libQt5Test.so.5.15.13
test_tournamentpair: /usr/lib/x86_64-linux-gnu/libQt5Core.so.5.15.13
test_tournamentpair: CMakeFiles/test_tournamentpair.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=/home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Linking CXX executable test_tournamentpair"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/test_tournamentpair.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/test_tournamentpair.dir/build: test_tournamentpair
.PHONY : CMakeFiles/test_tournamentpair.dir/build

CMakeFiles/test_tournamentpair.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/test_tournamentpair.dir/cmake_clean.cmake
.PHONY : CMakeFiles/test_tournamentpair.dir/clean

CMakeFiles/test_tournamentpair.dir/depend: test_tournamentpair_autogen/timestamp
	cd /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build /home/sasa/My_Projects/Graduation_Project/rough_hook/cutechess/build/CMakeFiles/test_tournamentpair.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : CMakeFiles/test_tournamentpair.dir/depend

