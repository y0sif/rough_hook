/****************************************************************************
** Meta object code from reading C++ file 'engineconfigurationdlg.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/engineconfigurationdlg.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'engineconfigurationdlg.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_EngineConfigurationDialog_t {
    QByteArrayData data[17];
    char stringdata0[245];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_EngineConfigurationDialog_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_EngineConfigurationDialog_t qt_meta_stringdata_EngineConfigurationDialog = {
    {
QT_MOC_LITERAL(0, 0, 25), // "EngineConfigurationDialog"
QT_MOC_LITERAL(1, 26, 17), // "detectionFinished"
QT_MOC_LITERAL(2, 44, 0), // ""
QT_MOC_LITERAL(3, 45, 13), // "browseCommand"
QT_MOC_LITERAL(4, 59, 13), // "setExecutable"
QT_MOC_LITERAL(5, 73, 4), // "file"
QT_MOC_LITERAL(6, 78, 16), // "browseWorkingDir"
QT_MOC_LITERAL(7, 95, 19), // "detectEngineOptions"
QT_MOC_LITERAL(8, 115, 15), // "restoreDefaults"
QT_MOC_LITERAL(9, 131, 19), // "onDetectionFinished"
QT_MOC_LITERAL(10, 151, 13), // "onEngineReady"
QT_MOC_LITERAL(11, 165, 12), // "onEngineQuit"
QT_MOC_LITERAL(12, 178, 12), // "onTabChanged"
QT_MOC_LITERAL(13, 191, 5), // "index"
QT_MOC_LITERAL(14, 197, 22), // "onNameOrCommandChanged"
QT_MOC_LITERAL(15, 220, 10), // "onAccepted"
QT_MOC_LITERAL(16, 231, 13) // "resizeColumns"

    },
    "EngineConfigurationDialog\0detectionFinished\0"
    "\0browseCommand\0setExecutable\0file\0"
    "browseWorkingDir\0detectEngineOptions\0"
    "restoreDefaults\0onDetectionFinished\0"
    "onEngineReady\0onEngineQuit\0onTabChanged\0"
    "index\0onNameOrCommandChanged\0onAccepted\0"
    "resizeColumns"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_EngineConfigurationDialog[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
      13,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       1,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    0,   79,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
       3,    0,   80,    2, 0x08 /* Private */,
       4,    1,   81,    2, 0x08 /* Private */,
       6,    0,   84,    2, 0x08 /* Private */,
       7,    0,   85,    2, 0x08 /* Private */,
       8,    0,   86,    2, 0x08 /* Private */,
       9,    0,   87,    2, 0x08 /* Private */,
      10,    0,   88,    2, 0x08 /* Private */,
      11,    0,   89,    2, 0x08 /* Private */,
      12,    1,   90,    2, 0x08 /* Private */,
      14,    0,   93,    2, 0x08 /* Private */,
      15,    0,   94,    2, 0x08 /* Private */,
      16,    0,   95,    2, 0x08 /* Private */,

 // signals: parameters
    QMetaType::Void,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void, QMetaType::QString,    5,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, QMetaType::Int,   13,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void EngineConfigurationDialog::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<EngineConfigurationDialog *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->detectionFinished(); break;
        case 1: _t->browseCommand(); break;
        case 2: _t->setExecutable((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 3: _t->browseWorkingDir(); break;
        case 4: _t->detectEngineOptions(); break;
        case 5: _t->restoreDefaults(); break;
        case 6: _t->onDetectionFinished(); break;
        case 7: _t->onEngineReady(); break;
        case 8: _t->onEngineQuit(); break;
        case 9: _t->onTabChanged((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 10: _t->onNameOrCommandChanged(); break;
        case 11: _t->onAccepted(); break;
        case 12: _t->resizeColumns(); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (EngineConfigurationDialog::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&EngineConfigurationDialog::detectionFinished)) {
                *result = 0;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject EngineConfigurationDialog::staticMetaObject = { {
    QMetaObject::SuperData::link<QDialog::staticMetaObject>(),
    qt_meta_stringdata_EngineConfigurationDialog.data,
    qt_meta_data_EngineConfigurationDialog,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *EngineConfigurationDialog::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *EngineConfigurationDialog::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_EngineConfigurationDialog.stringdata0))
        return static_cast<void*>(this);
    return QDialog::qt_metacast(_clname);
}

int EngineConfigurationDialog::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QDialog::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 13)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 13;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 13)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 13;
    }
    return _id;
}

// SIGNAL 0
void EngineConfigurationDialog::detectionFinished()
{
    QMetaObject::activate(this, &staticMetaObject, 0, nullptr);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
