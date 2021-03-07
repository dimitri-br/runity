using System.Runtime.InteropServices;
using System.Collections.Generic;
using System;
using UnityEngine;

/*
    This class will pool loaded DLL's to save from loading the same DLL multiple times.
    It will also manage the unloading of said DLL's (This must be called however)

    Attach this script to a gameobject.

*/

namespace runity_test
{
    /// <summary>
    /// This class contains some functions from the native windows library
    /// to help us load and release libraries at runtime.
    /// </summary>
    static class NativeMethods
    {
        [DllImport("kernel32.dll")]
        public static extern IntPtr LoadLibrary(string dllToLoad);

        [DllImport("kernel32.dll")]
        public static extern IntPtr GetProcAddress(IntPtr hModule, string procedureName);

        [DllImport("kernel32.dll")]
        public static extern bool FreeLibrary(IntPtr hModule);

        public static void Free(IntPtr ptr)
        {
            Marshal.FreeCoTaskMem(ptr);
        }
    }

    public class DLLPool: MonoBehaviour
    {
        public static Dictionary<string, IntPtr> dllPool = new Dictionary<string, IntPtr>();

        private static DLLPool instance;
        public static DLLPool Instance { get { return instance; } }

        private void Awake()
        {
            if (instance != null && instance != this)
            {
                Destroy(this.gameObject);
            }
            else
            {
                instance = this;
            }
        }
        
        /// <summary>
        /// This function loads a DLL from the stored dictionary.
        /// 
        /// This helps you save time by loading DLL's loaded already, rather than load them again
        /// </summary>
        /// <param name="dllName"></param>
        /// <returns></returns>
        public static IntPtr GetDLL(string dllName)
        {
            if (dllPool.ContainsKey(dllName))
            {
                return dllPool[dllName];
            }

            return IntPtr.Zero;
        }

        /// <summary>
        /// This function takes the name of the DLL to load
        /// 
        /// These DLL's are expected to be stored in the `Assets/Plugins/` folder
        /// </summary>
        /// <param name="dllName"></param>
        /// <returns></returns>
        public static IntPtr LoadDLL(string dllName)
        {
            if (GetDLL(dllName) != IntPtr.Zero)
            {
                return GetDLL(dllName);
            }

            //Get the path of the Game data folder
            string m_Path = Application.dataPath;

#if UNITY_EDITOR
            string path = m_Path + "/Plugins/" + dllName;
#else
            string path = m_Path + "/Plugins/x86_64/" + dllName;
#endif
            Debug.Log("Loading DLL: " + path);
            // Load the DLL library
            IntPtr loadedDLLPtr = NativeMethods.LoadLibrary(path);
            if (loadedDLLPtr == IntPtr.Zero)
            {
                Debug.LogError("Error! The library " + dllName + " couldn't be found!");
                return IntPtr.Zero;
            }
            else
            {
                Debug.Log("Successfully loaded DLL: " + path);
            }

            dllPool.Add(dllName, loadedDLLPtr);

            return loadedDLLPtr;
        }

        public static (Delegate, IntPtr) LoadFunctionFromDLL(string dllName, string functionName, Type delegateType)
        {
            try
            {
                if (GetDLL(dllName) != IntPtr.Zero)
                {
                    Debug.Log("DLL exists in pool, loading from pool...");
                    IntPtr dllPtr = GetDLL(dllName);

                    IntPtr functionPtr = NativeMethods.GetProcAddress(dllPtr, "update");
                    if (functionPtr == IntPtr.Zero)
                    {
                        Debug.LogWarning("Couldn't find function '" + functionName + "' in " + dllName + ", '" + functionName + "' in unity won't be run!");
                    }
                    Delegate function = Marshal.GetDelegateForFunctionPointer(functionPtr, delegateType);
                    Debug.Log("Loaded function '" + functionName + "' from " + dllName + " successfully!");
                    return (function, functionPtr);
                }
                else
                {
                    Debug.Log("DLL not loaded, loading DLL...");
                    IntPtr dllPtr = LoadDLL(dllName);

                    IntPtr functionPtr = NativeMethods.GetProcAddress(dllPtr, "update");
                    if (functionPtr == IntPtr.Zero)
                    {
                        Debug.LogWarning("Couldn't find function '" + functionName + "' in " + dllName + ", '" + functionName + "' in unity won't be run!");
                    }
                    Delegate function = Marshal.GetDelegateForFunctionPointer(functionPtr, delegateType);
                    Debug.Log("Loaded function '" + functionName + "' from " + dllName + " successfully!");
                    return (function, functionPtr);
                }
            }
            catch (Exception e)
            {
                Debug.Log("Error when loading function from DLL - " + e);
                return (null, IntPtr.Zero);
            }
        }

        public static void UnloadAll()
        {
            Debug.Log("Total DLLs currently loaded: " + dllPool.Count);
            // This is important - free all our loaded libraries
            foreach (string name in dllPool.Keys)
            {
                Debug.Log("Releasing DLL: " + name);
                NativeMethods.FreeLibrary(dllPool[name]);

            }
        }
    }
}


