"""
Feature Extraction Module
Extract features from files for ML-based threat detection
"""

import os
import hashlib
import math
import struct
import magic
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple
import numpy as np
import pefile
import yara
from collections import Counter
import structlog
from datetime import datetime, timedelta

from .config import Config
from .models import FileInfo

log = structlog.get_logger()


class FeatureExtractor:
    """Extract features from files for ML analysis"""
    
    def __init__(self, config: Config):
        self.config = config
        self._magic = None
        self._yara_rules = None
        self._setup_tools()
    
    def _setup_tools(self) -> None:
        """Setup analysis tools"""
        try:
            self._magic = magic.Magic(mime=True)
            log.info("Python-magic initialized successfully")
        except Exception as e:
            log.warning(f"Failed to initialize python-magic: {e}")
        
        try:
            # Initialize Yara rules (if available)
            self._load_yara_rules()
        except Exception as e:
            log.warning(f"Failed to initialize Yara: {e}")
    
    def _load_yara_rules(self) -> None:
        """Load Yara rules for malware detection"""
        # Create simple rules for demonstration
        rules_string = """
        rule SuspiciousStrings {
            strings:
                $suspicious1 = "cmd.exe"
                $suspicious2 = "powershell"
                $suspicious3 = "regsvr32"
                $suspicious4 = "rundll32"
                $suspicious5 = "wscript"
            condition:
                any of them
        }
        
        rule Packing {
            strings:
                $upx = "UPX"
                $packed = "UPX0"
                $packed2 = "UPX1"
            condition:
                any of them
        }
        
        rule SuspiciousAPI {
            strings:
                $api1 = "CreateRemoteThread"
                $api2 = "WriteProcessMemory"
                $api3 = "VirtualAllocEx"
                $api4 = "SetWindowsHookEx"
            condition:
                any of them
        }
        """
        try:
            self._yara_rules = yara.compile(source=rules_string)
            log.info("Yara rules compiled successfully")
        except Exception as e:
            log.warning(f"Failed to compile Yara rules: {e}")
            self._yara_rules = None
    
    def is_ready(self) -> bool:
        """Check if feature extractor is ready"""
        return self._magic is not None
    
    def get_supported_features(self) -> List[str]:
        """Get list of supported features"""
        return [
            "file_size", "file_hash", "file_type", "mime_type",
            "entropy", "file_extension_type", "is_executable",
            "has_digital_signature", "imports_count", "exports_count",
            "sections_count", "suspicious_strings", "pe_entropy",
            "is_packed", "compile_time", "has_debug_info", "has_tls",
            "resource_size", "version_info", "imported_dlls_count",
            "suspicious_imports", "file_age_days", "path_depth",
            "filename_length", "contains_spaces", "extension_risk",
            "is_system_file", "has_version_info", "company_info_present"
        ]
    
    async def extract_features(self, file_path: Path) -> Dict[str, Any]:
        """Extract all features from a file"""
        
        if not file_path.exists():
            raise FileNotFoundError(f"File not found: {file_path}")
        
        try:
            features = {}
            
            # Basic file information
            basic_features = await self._extract_basic_features(file_path)
            features.update(basic_features)
            
            # File content analysis
            content_features = await self._extract_content_features(file_path)
            features.update(content_features)
            
            # PE-specific features (if applicable)
            if features.get("is_executable", False):
                pe_features = await self._extract_pe_features(file_path)
                features.update(pe_features)
            
            # Yara analysis
            yara_features = await self._extract_yara_features(file_path)
            features.update(yara_features)
            
            log.debug(f"Extracted {len(features)} features from {file_path.name}")
            return features
            
        except Exception as e:
            log.error(f"Failed to extract features from {file_path}: {e}")
            raise
    
    async def _extract_basic_features(self, file_path: Path) -> Dict[str, Any]:
        """Extract basic file features"""
        
        stat = file_path.stat()
        now = datetime.utcnow()
        
        # File age in days
        file_age_days = (now - datetime.utcfromtimestamp(stat.st_mtime)).days
        
        # Path analysis
        path_parts = file_path.parts
        path_depth = len(path_parts)
        
        # Filename analysis
        filename = file_path.name
        filename_length = len(filename)
        contains_spaces = " " in filename
        
        # Extension analysis
        extension = file_path.suffix.lower()
        extension_risk = self._get_extension_risk(extension)
        
        # System file detection
        system_paths = [
            "/windows", "/system32", "/sys", "/proc", "/dev",
            "C:\\Windows", "C:\\System32"
        ]
        is_system_file = any(str(file_path).lower().startswith(path.lower()) for path in system_paths)
        
        return {
            "file_size": stat.st_size,
            "file_age_days": file_age_days,
            "path_depth": path_depth,
            "filename_length": filename_length,
            "contains_spaces": contains_spaces,
            "file_extension_type": extension or "none",
            "extension_risk": extension_risk,
            "is_system_file": is_system_file,
            "created_time": datetime.utcfromtimestamp(stat.st_ctime),
            "modified_time": datetime.utcfromtimestamp(stat.st_mtime),
            "accessed_time": datetime.utcfromtimestamp(stat.st_atime)
        }
    
    async def _extract_content_features(self, file_path: Path) -> Dict[str, Any]:
        """Extract content-based features"""
        
        features = {}
        
        try:
            with open(file_path, 'rb') as f:
                content = f.read()
            
            # File hash
            features["file_hash"] = hashlib.sha256(content).hexdigest()
            
            # File type detection
            if self._magic:
                try:
                    features["mime_type"] = self._magic.from_buffer(content)
                except:
                    features["mime_type"] = "application/octet-stream"
            else:
                features["mime_type"] = "application/octet-stream"
            
            # Entropy calculation
            features["entropy"] = self._calculate_entropy(content)
            
            # Executable detection
            features["is_executable"] = self._is_executable(content, file_path.suffix)
            
            # String analysis
            string_features = self._analyze_strings(content)
            features.update(string_features)
            
        except Exception as e:
            log.warning(f"Failed to extract content features from {file_path}: {e}")
            features.update({
                "file_hash": "",
                "mime_type": "unknown",
                "entropy": 0.0,
                "is_executable": False,
                "suspicious_strings": 0,
                "printable_strings": 0
            })
        
        return features
    
    async def _extract_pe_features(self, file_path: Path) -> Dict[str, Any]:
        """Extract PE file features"""
        
        features = {}
        
        try:
            pe = pefile.PE(str(file_path))
            
            # Basic PE information
            features["compile_time"] = pe.FILE_HEADER.TimeDateStamp
            
            # Sections analysis
            if hasattr(pe, 'sections'):
                features["sections_count"] = len(pe.sections)
                
                # Calculate entropy for each section
                entropies = []
                for section in pe.sections:
                    try:
                        section_entropy = self._calculate_entropy(section.get_data())
                        entropies.append(section_entropy)
                    except:
                        entropies.append(0.0)
                
                features["pe_entropy"] = np.mean(entropies) if entropies else 0.0
            else:
                features["sections_count"] = 0
                features["pe_entropy"] = 0.0
            
            # Imports analysis
            if hasattr(pe, 'DIRECTORY_ENTRY_IMPORT'):
                imports = []
                imported_dlls = set()
                
                for entry in pe.DIRECTORY_ENTRY_IMPORT:
                    imported_dlls.add(entry.dll.decode('utf-8', errors='ignore'))
                    for imp in entry.imports:
                        if imp.name:
                            imports.append(imp.name.decode('utf-8', errors='ignore'))
                
                features["imports_count"] = len(imports)
                features["imported_dlls_count"] = len(imported_dlls)
                features["suspicious_imports"] = self._count_suspicious_imports(imports)
            else:
                features["imports_count"] = 0
                features["imported_dlls_count"] = 0
                features["suspicious_imports"] = 0
            
            # Exports analysis
            if hasattr(pe, 'DIRECTORY_ENTRY_EXPORT'):
                features["exports_count"] = len(pe.DIRECTORY_ENTRY_EXPORT.symbols)
            else:
                features["exports_count"] = 0
            
            # Digital signature
            features["has_digital_signature"] = hasattr(pe, 'OPTIONAL_HEADER') and \
                                                pe.OPTIONAL_HEADER.DATA_DIRECTORY[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_SECURITY']].Size > 0
            
            # Debug information
            features["has_debug_info"] = hasattr(pe, 'DIRECTORY_ENTRY_DEBUG')
            
            # TLS (Thread Local Storage)
            features["has_tls"] = hasattr(pe, 'DIRECTORY_ENTRY_TLS')
            
            # Version information
            features["has_version_info"] = hasattr(pe, 'VS_VERSIONINFO')
            
            # Company information
            features["company_info_present"] = self._has_company_info(pe)
            
            # Resource size
            if hasattr(pe, 'DIRECTORY_ENTRY_RESOURCE'):
                features["resource_size"] = sum(
                    entry.directory.size for entry in pe.DIRECTORY_ENTRY_RESOURCE.entries
                )
            else:
                features["resource_size"] = 0
            
            # Packing detection
            features["is_packed"] = self._is_packed(pe, features.get("pe_entropy", 0))
            
            pe.close()
            
        except Exception as e:
            log.warning(f"Failed to extract PE features from {file_path}: {e}")
            # Set default values for PE features
            features.update({
                "compile_time": 0,
                "sections_count": 0,
                "pe_entropy": 0.0,
                "imports_count": 0,
                "imported_dlls_count": 0,
                "suspicious_imports": 0,
                "exports_count": 0,
                "has_digital_signature": False,
                "has_debug_info": False,
                "has_tls": False,
                "has_version_info": False,
                "company_info_present": False,
                "resource_size": 0,
                "is_packed": False
            })
        
        return features
    
    async def _extract_yara_features(self, file_path: Path) -> Dict[str, Any]:
        """Extract Yara-based features"""
        
        features = {
            "yara_matches": 0,
            "suspicious_strings_yara": 0,
            "packing_detected": 0,
            "suspicious_api": 0
        }
        
        if self._yara_rules is None:
            return features
        
        try:
            matches = self._yara_rules.match(str(file_path))
            
            features["yara_matches"] = len(matches)
            
            for match in matches:
                if match.rule == "SuspiciousStrings":
                    features["suspicious_strings_yara"] = 1
                elif match.rule == "Packing":
                    features["packing_detected"] = 1
                elif match.rule == "SuspiciousAPI":
                    features["suspicious_api"] = 1
                    
        except Exception as e:
            log.warning(f"Yara analysis failed for {file_path}: {e}")
        
        return features
    
    def _calculate_entropy(self, data: bytes) -> float:
        """Calculate Shannon entropy of data"""
        
        if not data:
            return 0.0
        
        # Count byte frequencies
        byte_counts = Counter(data)
        total_bytes = len(data)
        
        # Calculate entropy
        entropy = 0.0
        for count in byte_counts.values():
            probability = count / total_bytes
            entropy -= probability * math.log2(probability)
        
        return entropy
    
    def _is_executable(self, content: bytes, extension: str) -> bool:
        """Check if file is executable"""
        
        executable_extensions = {
            '.exe', '.dll', '.sys', '.drv', '.ocx',
            '.scr', '.cpl', '.com', '.pif', '.msi',
            '.bat', '.cmd', '.ps1', '.vbs', '.js',
            '.wsf', '.jar', '.app', '.deb', '.rpm'
        }
        
        # Check by extension
        if extension.lower() in executable_extensions:
            return True
        
        # Check by magic bytes
        if len(content) >= 2:
            # PE executables
            if content[:2] == b'MZ':
                return True
            # ELF executables
            if content[:4] == b'\x7fELF':
                return True
            # Mach-O executables
            if content[:4] in [b'\xfe\xed\xfa\xce', b'\xfe\xed\xfa\xcf', b'\xce\xfa\xed\xfe', b'\xcf\xfa\xed\xfe']:
                return True
        
        return False
    
    def _analyze_strings(self, content: bytes) -> Dict[str, int]:
        """Analyze strings in file content"""
        
        try:
            # Extract printable strings (min length 4)
            strings = []
            current_string = ""
            
            for byte in content:
                if 32 <= byte <= 126:  # Printable ASCII
                    current_string += chr(byte)
                else:
                    if len(current_string) >= 4:
                        strings.append(current_string.lower())
                    current_string = ""
            
            # Add the last string if it meets the criteria
            if len(current_string) >= 4:
                strings.append(current_string.lower())
            
            # Count suspicious strings
            suspicious_keywords = [
                'cmd.exe', 'powershell', 'regsvr32', 'rundll32', 'wscript',
                'createremotethread', 'writeprocessmemory', 'virtualallocex',
                'setwindowshookex', 'getprocaddress', 'loadlibrary',
                'createprocess', 'shellcode', 'base64', 'xor', 'decrypt',
                'password', 'hack', 'crack', 'keylog', 'steal'
            ]
            
            suspicious_count = 0
            for string in strings:
                for keyword in suspicious_keywords:
                    if keyword in string:
                        suspicious_count += 1
                        break
            
            return {
                "suspicious_strings": suspicious_count,
                "printable_strings": len(strings)
            }
            
        except Exception as e:
            log.warning(f"String analysis failed: {e}")
            return {"suspicious_strings": 0, "printable_strings": 0}
    
    def _get_extension_risk(self, extension: str) -> float:
        """Get risk score for file extension (0-10)"""
        
        risk_scores = {
            '.exe': 8.0,
            '.dll': 7.0,
            '.scr': 9.0,
            '.bat': 6.0,
            '.cmd': 6.0,
            '.ps1': 7.0,
            '.vbs': 8.0,
            '.js': 5.0,
            '.jar': 6.0,
            '.com': 9.0,
            '.pif': 9.0,
            '.msi': 5.0,
            '.sys': 7.0,
            '.drv': 7.0,
            '.ocx': 6.0,
            '.cpl': 6.0,
            '.wsf': 7.0,
            '.app': 6.0,
            '.deb': 4.0,
            '.rpm': 4.0,
        }
        
        return risk_scores.get(extension.lower(), 1.0)
    
    def _count_suspicious_imports(self, imports: List[str]) -> int:
        """Count suspicious API imports"""
        
        suspicious_apis = [
            'CreateRemoteThread', 'WriteProcessMemory', 'VirtualAllocEx',
            'SetWindowsHookEx', 'GetProcAddress', 'LoadLibrary',
            'CreateProcess', 'ShellExecute', 'WinExec',
            'VirtualProtect', 'VirtualAlloc', 'HeapCreate',
            'CreateFile', 'DeleteFile', 'MoveFile',
            'RegCreateKey', 'RegSetValue', 'RegDeleteValue',
            'Socket', 'Connect', 'Send', 'Recv',
            'InternetOpen', 'InternetConnect', 'HttpOpenRequest',
            'GetAsyncKeyState', 'GetWindowText', 'GetForegroundWindow'
        ]
        
        count = 0
        for import_name in imports:
            for suspicious_api in suspicious_apis:
                if suspicious_api.lower() in import_name.lower():
                    count += 1
                    break
        
        return count
    
    def _has_company_info(self, pe) -> bool:
        """Check if PE has company information"""
        
        try:
            if hasattr(pe, 'FileInfo'):
                for file_info in pe.FileInfo:
                    if file_info.Key.decode() == 'StringFileInfo':
                        for string_table in file_info.StringTable:
                            for entry in string_table.entries.items():
                                if 'CompanyName' in entry[0]:
                                    return True
            return False
        except:
            return False
    
    def _is_packed(self, pe, entropy: float) -> bool:
        """Check if executable is packed"""
        
        # High entropy suggests packing
        if entropy > 7.5:
            return True
        
        # Check for common packer names in imports
        try:
            if hasattr(pe, 'DIRECTORY_ENTRY_IMPORT'):
                for entry in pe.DIRECTORY_ENTRY_IMPORT:
                    dll_name = entry.dll.decode('utf-8', errors='ignore').lower()
                    if any(packer in dll_name for packer in ['upx', 'aspack', 'pecompact', 'mpress']):
                        return True
        except:
            pass
        
        return False