/****************************************************************************
** Meta object code from reading C++ file 'hooklenswindow.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/hooklenswindow.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'hooklenswindow.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_HookLensWindow_t {
    QByteArrayData data[12];
    char stringdata0[149];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_HookLensWindow_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_HookLensWindow_t qt_meta_stringdata_HookLensWindow = {
    {
QT_MOC_LITERAL(0, 0, 14), // "HookLensWindow"
QT_MOC_LITERAL(1, 15, 14), // "fenStringReady"
QT_MOC_LITERAL(2, 30, 0), // ""
QT_MOC_LITERAL(3, 31, 3), // "fen"
QT_MOC_LITERAL(4, 35, 13), // "bestMoveReady"
QT_MOC_LITERAL(5, 49, 10), // "fromSquare"
QT_MOC_LITERAL(6, 60, 8), // "toSquare"
QT_MOC_LITERAL(7, 69, 14), // "resetRequested"
QT_MOC_LITERAL(8, 84, 17), // "onAddPhotoClicked"
QT_MOC_LITERAL(9, 102, 13), // "onSendClicked"
QT_MOC_LITERAL(10, 116, 17), // "onBestMoveClicked"
QT_MOC_LITERAL(11, 134, 14) // "onResetClicked"

    },
    "HookLensWindow\0fenStringReady\0\0fen\0"
    "bestMoveReady\0fromSquare\0toSquare\0"
    "resetRequested\0onAddPhotoClicked\0"
    "onSendClicked\0onBestMoveClicked\0"
    "onResetClicked"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_HookLensWindow[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       7,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       3,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    1,   49,    2, 0x06 /* Public */,
       4,    2,   52,    2, 0x06 /* Public */,
       7,    0,   57,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
       8,    0,   58,    2, 0x08 /* Private */,
       9,    0,   59,    2, 0x08 /* Private */,
      10,    0,   60,    2, 0x08 /* Private */,
      11,    0,   61,    2, 0x08 /* Private */,

 // signals: parameters
    QMetaType::Void, QMetaType::QString,    3,
    QMetaType::Void, QMetaType::QString, QMetaType::QString,    5,    6,
    QMetaType::Void,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void HookLensWindow::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<HookLensWindow *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->fenStringReady((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 1: _t->bestMoveReady((*reinterpret_cast< const QString(*)>(_a[1])),(*reinterpret_cast< const QString(*)>(_a[2]))); break;
        case 2: _t->resetRequested(); break;
        case 3: _t->onAddPhotoClicked(); break;
        case 4: _t->onSendClicked(); break;
        case 5: _t->onBestMoveClicked(); break;
        case 6: _t->onResetClicked(); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (HookLensWindow::*)(const QString & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&HookLensWindow::fenStringReady)) {
                *result = 0;
                return;
            }
        }
        {
            using _t = void (HookLensWindow::*)(const QString & , const QString & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&HookLensWindow::bestMoveReady)) {
                *result = 1;
                return;
            }
        }
        {
            using _t = void (HookLensWindow::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&HookLensWindow::resetRequested)) {
                *result = 2;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject HookLensWindow::staticMetaObject = { {
    QMetaObject::SuperData::link<QWidget::staticMetaObject>(),
    qt_meta_stringdata_HookLensWindow.data,
    qt_meta_data_HookLensWindow,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *HookLensWindow::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *HookLensWindow::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_HookLensWindow.stringdata0))
        return static_cast<void*>(this);
    return QWidget::qt_metacast(_clname);
}

int HookLensWindow::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QWidget::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 7)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 7;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 7)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 7;
    }
    return _id;
}

// SIGNAL 0
void HookLensWindow::fenStringReady(const QString & _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}

// SIGNAL 1
void HookLensWindow::bestMoveReady(const QString & _t1, const QString & _t2)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t2))) };
    QMetaObject::activate(this, &staticMetaObject, 1, _a);
}

// SIGNAL 2
void HookLensWindow::resetRequested()
{
    QMetaObject::activate(this, &staticMetaObject, 2, nullptr);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
