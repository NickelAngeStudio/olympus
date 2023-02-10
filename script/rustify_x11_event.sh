####################################################
# FILE
# rustify_x11_event.sh
#
# DESCRIPTION
# Generate Xlib.h XEvent union into Rust format.
#
# PARAMETERS
# $1 : Path to Xlib.h
#
# USAGE
# $ bash rustify_x11_event.sh {path_to_Xlib.h} > {destination.rs}	
#
# NOTE
# 
#
# REFERENCES
# https://docs.rs/x11/latest/x11
# https://superuser.com/questions/1001973/bash-find-string-index-position-of-substring
#
# COPYRIGHT
# MIT
#
# NickelAnge.Studio
# 2023-02-02
####################################################

# Verify that the file is indeed Xlib.h
if [[ "$1" != *"Xlib.h"* ]]; then
	echo "Error : First argument must be path to Xlib.h file!" 
	echo "USAGE : bash rustify_x11_event.sh {path_to_Xlib.h} > {destination.rs}"
	exit 1
fi

# Rust file header
echo "// Generated with \"script/rustify_x11_event.sh\""
echo "// NOTE: All struct and union members name start with \"_\" to prevent conflict with Rust reserved words."
echo ""
echo "use std::os::raw::{ c_int, c_long, c_uint, c_ulong, c_char, c_uchar, c_short, c_void };"
echo ""


echo "// Types definition (ref : https://docs.rs/x11/latest/x11)"
echo "pub type Time = c_ulong;"
echo "pub type XID = c_ulong;"
echo "pub type Atom = XID;"
echo "pub type Colormap = XID;"
echo "pub type Drawable = XID;"
echo "pub type Window = XID;"
echo "pub type Display = XID;"
echo ""
echo "/// Union 'data' of XClientMessageEvent struct."
echo "#[repr(C)]"
echo "#[derive(Debug, Clone, Copy, PartialEq)]"
echo "pub struct XClientMessageEvent_data {"
echo "pub _b : [c_char; 20],"
echo "pub _s : [c_short; 10],"
echo "pub _l : [c_long; 5]"
echo "}"

# Flag that tell if we are at xevent definition lines start
xevent_start=0

# Flag that tell if we are at xevent definition lines end
xevent_end=0

# Tell if we are in a struct block
struct_block=0

# Tell if we are in a union block
union_block=0

# Tell if we are at struct end
struct_end=0

# Content of struct/union block
body_content=""


# Remove variable junk given by AWK
remove_var_junk() {
	if [[ "$1" == *"/"* ]]; then
		echo ""
	else
		echo $(echo $(echo $1 | sed 's/;//g') | sed 's/,//g')
	fi
}

# Get position of char $1 in $2. REF : https://superuser.com/questions/1001973/bash-find-string-index-position-of-substring
get_char_position() {
	t=$2
	searchstring=$1

	rest=${t#*$searchstring}
	echo $(( ${#t} - ${#rest} - ${#searchstring} ))
}

# Echo $2 to body content with $1 as type.
echo_to_body_content() {

	if [[ $2 != "" ]]; then
		# Mutable pointer
		if [[ "$2" == *"*"* ]]; then
			v_name="\tpub _$(echo $2 | sed 's/*//g')"
			body_content=$(echo "$body_content$v_name:*mut $1,\n")
		
		# Array
		elif [[ "$2" == *"["* ]]; then
			position=$(get_char_position "[" "$2")
	
			v_name="\tpub _${2:0:position}"
			#v_size=$($(echo ${2:position:255} | sed 's/[//g') | sed 's/]//g')
			v_size=${2:position:255}
			v_size=$(echo $v_size | sed 's/\[//g')
			v_size=$(echo $v_size | sed 's/\]//g')
			
			body_content=$(echo "$body_content$v_name:[$1; $v_size],\n")
		
		# Variable
		else
			v_name="\tpub _$2"
			body_content=$(echo "$body_content$v_name:$1,\n")
		fi
	fi

}

# Write struct body content. $1 is type. $2 is index of V (2 or 3). 
write_struct_body_content() {

	if [ $2 == 2 ]; then
		echo_to_body_content "$1" "$v2"
		echo_to_body_content "$1" "$v3"
		echo_to_body_content "$1" "$v4"
		echo_to_body_content "$1" "$v5"
	else
		echo_to_body_content "$1" "$v3"
		echo_to_body_content "$1" "$v4"
		echo_to_body_content "$1" "$v5"
	fi
}

# Parse struct line into body content. Note : Code is ugly. $1 is line.
parse_struct_body_content() {

	v1=$(remove_var_junk $(echo $line | awk '{print $1}'))
	v2=$(remove_var_junk $(echo $line | awk '{print $2}'))
	v3=$(remove_var_junk $(echo $line | awk '{print $3}'))
	v4=$(remove_var_junk $(echo $line | awk '{print $4}'))
	v5=$(remove_var_junk $(echo $line | awk '{print $5}'))

	if [[ "$line" == *"unsigned long"* ]]; then
		write_struct_body_content "c_ulong" 3
	
	elif [[ "$line" == *"unsigned int"* ]]; then
		write_struct_body_content "c_uint" 3
	
	elif [[ "$line" == *"unsigned char"* ]]; then
		write_struct_body_content "c_uchar" 3
	
	elif [[ "$line" == *"void"* ]]; then
		write_struct_body_content "c_void" 2
		
	elif [[ "$line" == *"C++"* ]]; then
		# Ignore C++
		:
		
	elif [[ "$line" == *"union {"* ]]; then
		# XClientMessageEvent union
		v_name="\tpub _data"
		body_content=$(echo "$body_content$v_name:XClientMessageEvent_data,\n")

		
	elif [[ "$line" == *"char b[20];"* ]]; then
		# Ignore union rest
		:
		
	elif [[ "$line" == *"short s[10];"* ]]; then
		# Ignore union rest
		:
		
	elif [[ "$line" == *"long l[5];"* ]]; then
		# Ignore union rest
		:
		
	elif [[ "$line" == *"} data"* ]]; then
		# Ignore union rest
		:
		
	elif [[ "$line" == *"{"* ]]; then
		# Ignore struct opening bracket
		:
	
	elif [[ "$line" == *"Bool"* ]]; then
		write_struct_body_content "bool" 2
	
	elif [[ "$line" == *"Display"* ]]; then
		write_struct_body_content "Display" 2
	
	elif [[ "$line" == *"Window"* ]]; then
		write_struct_body_content "Window" 2
	
	elif [[ "$line" == *"Time"* ]]; then
		write_struct_body_content "Time" 2
		
	elif [[ "$line" == *"NotifyPointer"* ]]; then
		# Ignore comment rest
		:
		
	elif [[ "$line" == *"int"* ]]; then
		write_struct_body_content "c_int" 2
		
	elif [[ "$line" == *"long"* ]]; then
		write_struct_body_content "c_long" 2
		
	elif [[ "$line" == *"char"* ]]; then
		write_struct_body_content "c_char" 2
	
	elif [[ "$line" == *"Drawable"* ]]; then
		write_struct_body_content "Drawable" 2
	
	elif [[ "$line" == *"Atom"* ]]; then
		write_struct_body_content "Atom" 2
	elif [[ "$line" == *"*"* ]]; then
		# ignore comment line
		:
		
	elif [[ "$line" == *"#if defined"* ]]; then
		# ignore define line
		:
	elif [[ "$line" == *"#else"* ]]; then
		# ignore define line
		:
	elif [[ "$line" == *"#endif"* ]]; then
		# ignore define line
		:
	elif [[ "$block_type" == *"union"* ]]; then
		# write union lines
		write_struct_body_content "$v1" 2
	else
		echo "Unsupported : $line"
		exit 1
	fi

}

# Read the Xlib.h file
while read -r line
do
	# If we reached the end of xevent definition, exit succesfully
	if [ $xevent_end == 1 ]; then
		exit 0
	fi
	
	# Generate code only if at _XEVENT_ block
	if [ $xevent_start == 1 ]; then
		# End of struct block
		if [[ "$line" == *"}"* && $struct_block == 1 && "$line" != *"} data;"* ]]; then
			# End struct block
			struct_block=0
			
			# Get struct name
			struct_name=$(echo $line | awk '{print $2}')
			struct_name=$(echo $struct_name | sed 's/;//g')
						
			# Output struct in rust format
			echo "pub $block_type $struct_name {"
			printf "$body_content" 
			echo "}"
			echo ""
			
			# Reset body_content
			body_content=""
		fi
		
		# Each line are for struct block
		if [ $struct_block == 1 ]; then
			parse_struct_body_content "$line"
		fi
		
		# Beginning of struct block
		if [[ "$line" == *"typedef struct"* && $struct_block == 0 ]]; then
			struct_block=1
			block_type="struct"
			echo "#[repr(C)]"
			echo "#[derive(Debug, Clone, Copy, PartialEq)]"
			
			
		elif [[ "$line" == *"typedef union"* && $struct_block == 0 ]]; then
			struct_block=1
			block_type="union"
			echo "#[repr(C)]"
			echo "#[derive(Clone, Copy)]"
		
		# Type Alias
		elif [[ "$line" == *"typedef "* ]]; then
			alias_name=$(remove_var_junk $(echo $line | awk '{print $3}'))
			alias_type=$(remove_var_junk $(echo $line | awk '{print $2}'))
			echo "pub type $alias_name = $alias_type;"
		fi
		
		
		
		
		
	fi
	

	# Start point of XEVENT definition
	if [[ "$line" == *"#ifndef _XEVENT_"* ]]; then
		xevent_start=1
	fi
	
	# End point of XEVENT definition
	if [[ "$line" == *"#define XAllocID(dpy)"* ]]; then
		xevent_end=1
	fi
	
	
	

done < "$1"
