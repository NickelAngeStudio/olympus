####################################################
# FILE
# rustify_x11_constants.sh
#
# DESCRIPTION
# Generate X.h #define variable into Rust contants format.
#
# PARAMETERS
# $1 : Path to X.h
#
# USAGE
# $ bash rustify_x11_contants.sh {path_to_X.h} > {destination.rs}	
#
# NOTE
# A trailing */ at line 204 must be removed manually after generate.
#
# COPYRIGHT
# MIT
#
# NickelAnge.Studio
# 2023-02-01
####################################################

# Verify that the file is indeed X.h
if [[ "$1" != *"X.h"* ]]; then
	echo "Error : First argument must be path to X.h file!" 
	echo "USAGE : bash rustify_x11_contants.sh {path_to_X.h} > {destination.rs}"
	exit 1
fi

# Rust file header
echo "// Generated with \"script/rustify_x11_constants.sh\""
echo "use std::os::raw::{ c_int, c_long };"
echo ""

# Write the variable and value in Rust format
write_rust_variable_and_value() {

	if [[ "$1" != *"X_H"* ]]; then	# Make sure we don't add empty define
		if [[ "$2" == *"(1<<"* ]]; then
			local value=$(echo "pub const $1:c_long=1 << ${2:4:50}" | awk '{ print substr( $0, 1, length($0)-1 ) }')
			echo "$value;"
		elif [[ "$2" == *"(1L<<"* ]]; then
			local value=$(echo "pub const $1:c_long=1 << ${2:5:50}" | awk '{ print substr( $0, 1, length($0)-1 ) }')
			echo "$value;"
		elif [[ "$2" == *"0L"* ]]; then
			echo "pub const $1:c_long=0;"
		elif [[ "$2" == *"1L"* ]]; then
			echo "pub const $1:c_long=0;"
		elif [[ "$2" == *"(int)"* ]]; then
			echo "pub const $1:c_int=${2:5:50} as i32;"
		else
			echo "pub const $1:c_int=$2;"
		fi
	fi

}

# Read the X.h file
while read -r line
do
  
  
  # Parse file according to first 2 characters of line
  case ${line:0:2} in

	# Global variable definition start with #d
     "#d")
	  # Extract variable name      
      varname=$(echo $line | awk '{print $2}')
      
      # Extract value
      value=$(echo $line | awk '{print $3}')
      
      write_rust_variable_and_value $varname $value
      
      
      ;;
      
    # Comments start with /*
    "/*")
      echo "$line"
      ;;
      
    # Event follow up
    " *" | "**" | "* " | "*/")
      echo "$line"
      ;;

    
	
    *)
	  # Close comment line
      if [[ "$line" == *"*/"* ]]; then
        # Remove random trash
        if [[ "$line" != *"typedef"* && "$line" != *"ChangeWindowAttributes"* && "$line" != *"class"* && "$line" != *"#endif"* ]]; then
          if [[ "$line" != "*/" ]]; then
        	  echo "$line"
          fi
  	    fi
      fi
      ;;
  esac 
  
done < "$1"
